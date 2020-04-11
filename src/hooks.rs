use crate::base::*;
use crate::utils::*;

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
    println!("ASSERT: {} {}", line, file_name);
    loop {}
}