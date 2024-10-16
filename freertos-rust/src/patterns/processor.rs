use crate::base::*;
use crate::mutex::*;
use crate::prelude::v1::*;
use crate::queue::*;
use crate::units::*;

pub type SharedClientWithReplyQueue<O> = Arc<ClientWithReplyQueue<O>>;
pub type Client<I> = ProcessorClient<I, ()>;
pub type ClientWithReplies<I, O> = ProcessorClient<I, SharedClientWithReplyQueue<O>>;

pub trait ReplyableMessage {
    fn reply_to_client_id(&self) -> Option<usize>;
}

#[derive(Copy, Clone)]
pub struct InputMessage<I>
where
    I: Copy + Send,
{
    val: I,
    reply_to_client_id: Option<usize>,
}

impl<I> InputMessage<I>
where
    I: Copy + Send,
{
    pub fn request(val: I) -> Self {
        InputMessage {
            val: val,
            reply_to_client_id: None,
        }
    }

    pub fn request_with_reply(val: I, client_id: usize) -> Self {
        InputMessage {
            val: val,
            reply_to_client_id: Some(client_id),
        }
    }

    pub fn get_val(&self) -> I {
        self.val
    }
}

impl<I> ReplyableMessage for InputMessage<I>
where
    I: Copy + Send,
{
    fn reply_to_client_id(&self) -> Option<usize> {
        self.reply_to_client_id
    }
}

pub struct Processor<I, O>
where
    I: ReplyableMessage + Copy + Send,
    O: Copy + Send,
{
    queue: Arc<Queue<I>>,
    inner: Arc<Mutex<ProcessorInner<O>>>,
}

impl<I, O> Processor<I, O>
where
    I: ReplyableMessage + Copy + Send,
    O: Copy + Send,
{
    pub fn new(queue_size: usize) -> Result<Self, FreeRtosError> {
        let p = ProcessorInner {
            clients: Vec::new(),
            next_client_id: 1,
        };
        let p = Arc::new(Mutex::new(p)?);
        let p = Processor {
            queue: Arc::new(Queue::new(queue_size)?),
            inner: p,
        };
        Ok(p)
    }

    pub fn new_client(&self) -> Result<Client<I>, FreeRtosError> {
        let c = ProcessorClient {
            processor_queue: Arc::downgrade(&self.queue),
            client_reply: (),
        };

        Ok(c)
    }

    pub fn new_client_with_reply<D: DurationTicks>(
        &self,
        client_receive_queue_size: usize,
        max_wait: D,
    ) -> Result<ProcessorClient<I, SharedClientWithReplyQueue<O>>, FreeRtosError> {
        if client_receive_queue_size == 0 {
            return Err(FreeRtosError::InvalidQueueSize);
        }

        let client_reply = {
            let mut processor = self.inner.lock(max_wait)?;

            let c = ClientWithReplyQueue {
                id: processor.next_client_id,
                processor_inner: self.inner.clone(),
                receive_queue: Queue::new(client_receive_queue_size)?,
            };

            let c = Arc::new(c);
            processor.clients.push((c.id, Arc::downgrade(&c)));

            processor.next_client_id += 1;

            c
        };

        let c = ProcessorClient {
            processor_queue: Arc::downgrade(&self.queue),
            client_reply: client_reply,
        };

        Ok(c)
    }

    pub fn get_receive_queue(&self) -> &Queue<I> {
        &*self.queue
    }

    pub fn reply<D: DurationTicks>(
        &self,
        received_message: I,
        reply: O,
        max_wait: D,
    ) -> Result<bool, FreeRtosError> {
        if let Some(client_id) = received_message.reply_to_client_id() {
            let inner = self.inner.lock(max_wait)?;
            if let Some(client) = inner
                .clients
                .iter()
                .flat_map(|ref x| x.1.upgrade().into_iter())
                .find(|x| x.id == client_id)
            {
                client
                    .receive_queue
                    .send(reply, max_wait)
                    .map_err(|err| err.error())?;
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl<I, O> Processor<InputMessage<I>, O>
where
    I: Copy + Send,
    O: Copy + Send,
{
    pub fn reply_val<D: DurationTicks>(
        &self,
        received_message: InputMessage<I>,
        reply: O,
        max_wait: D,
    ) -> Result<bool, FreeRtosError> {
        self.reply(received_message, reply, max_wait)
    }
}

struct ProcessorInner<O>
where
    O: Copy + Send,
{
    clients: Vec<(usize, Weak<ClientWithReplyQueue<O>>)>,
    next_client_id: usize,
}

impl<O> ProcessorInner<O>
where
    O: Copy + Send,
{
    fn remove_client_reply(&mut self, client: &ClientWithReplyQueue<O>) {
        self.clients.retain(|ref x| x.0 != client.id)
    }
}

pub struct ProcessorClient<I, C>
where
    I: ReplyableMessage + Copy + Send,
{
    processor_queue: Weak<Queue<I>>,
    client_reply: C,
}

impl<I, O> ProcessorClient<I, O>
where
    I: ReplyableMessage + Copy + Send,
{
    pub fn send<D: DurationTicks>(&self, message: I, max_wait: D) -> Result<(), FreeRtosError> {
        let processor_queue = self
            .processor_queue
            .upgrade()
            .ok_or(FreeRtosError::ProcessorHasShutDown)?;
        processor_queue
            .send(message, max_wait)
            .map_err(|err| err.error())?;
        Ok(())
    }

    pub fn send_from_isr(
        &self,
        context: &mut crate::isr::InterruptContext,
        message: I,
    ) -> Result<(), FreeRtosError> {
        let processor_queue = self
            .processor_queue
            .upgrade()
            .ok_or(FreeRtosError::ProcessorHasShutDown)?;
        processor_queue
            .send_from_isr(context, message)
            .map_err(|err| err.error())
    }
}

impl<I> ProcessorClient<InputMessage<I>, ()>
where
    I: Copy + Send,
{
    pub fn send_val<D: DurationTicks>(&self, val: I, max_wait: D) -> Result<(), FreeRtosError> {
        self.send(InputMessage::request(val), max_wait)
    }

    pub fn send_val_from_isr(
        &self,
        context: &mut crate::isr::InterruptContext,
        val: I,
    ) -> Result<(), FreeRtosError> {
        self.send_from_isr(context, InputMessage::request(val))
    }
}

impl<I, O> ProcessorClient<I, SharedClientWithReplyQueue<O>>
where
    I: ReplyableMessage + Copy + Send,
    O: Copy + Send,
{
    pub fn call<D: DurationTicks>(&self, message: I, max_wait: D) -> Result<O, FreeRtosError> {
        self.send(message, max_wait)?;
        self.client_reply.receive_queue.receive(max_wait)
    }

    pub fn get_receive_queue(&self) -> &Queue<O> {
        &self.client_reply.receive_queue
    }
}

impl<I, O> ProcessorClient<InputMessage<I>, SharedClientWithReplyQueue<O>>
where
    I: Copy + Send,
    O: Copy + Send,
{
    pub fn send_val<D: DurationTicks>(&self, val: I, max_wait: D) -> Result<(), FreeRtosError> {
        self.send(InputMessage::request(val), max_wait)
    }

    pub fn call_val<D: DurationTicks>(&self, val: I, max_wait: D) -> Result<O, FreeRtosError> {
        let reply = self.call(
            InputMessage::request_with_reply(val, self.client_reply.id),
            max_wait,
        )?;
        Ok(reply)
    }
}

impl<I, C> Clone for ProcessorClient<I, C>
where
    I: ReplyableMessage + Copy + Send,
    C: Clone,
{
    fn clone(&self) -> Self {
        ProcessorClient {
            processor_queue: self.processor_queue.clone(),
            client_reply: self.client_reply.clone(),
        }
    }
}

pub struct ClientWithReplyQueue<O>
where
    O: Copy + Send,
{
    id: usize,
    processor_inner: Arc<Mutex<ProcessorInner<O>>>,
    receive_queue: Queue<O>,
}

impl<O> Drop for ClientWithReplyQueue<O>
where
    O: Copy + Send,
{
    fn drop(&mut self) {
        if let Ok(mut p) = self.processor_inner.lock(Duration::ms(1000)) {
            p.remove_client_reply(&self);
        }
    }
}
