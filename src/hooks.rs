use crate::base::*;
use crate::utils::*;
use crate::prelude::v1::String;

#[allow(unused_doc_comments)]

#[no_mangle]
pub extern "C" fn freerots_rs_assert_called(line: FreeRtosUBaseType, file_name_ptr: FreeRtosCharPtr) {
    /**
    void vAssertCalled( unsigned long ulLine, const char * const pcFileName )

    static BaseType_t xPrinted = pdFALSE;
	volatile uint32_t ulSetToNonZeroInDebuggerToContinue = 0;

	/* Called if an assertion passed to configASSERT() fails.  See
	http://www.freertos.org/a00110.html#configASSERT for more information. */

	/* Parameters are not used. */
	( void ) ulLine;
	( void ) pcFileName;

	printf( "ASSERT! Line %ld, file %s, GetLastError() %ld\r\n", ulLine, pcFileName, GetLastError() );
*/
    let file_name: String;
    unsafe {
        file_name = str_from_c_string(file_name_ptr).unwrap();
    }
    // we can't print without std yet.
    // TODO: make the macro work for debug UART? Or use Panic here?
    // println!("ASSERT: {} {}", line, file_name);
    loop {}
}