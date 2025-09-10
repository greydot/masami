use core::panic::PanicInfo;

use super::console::Console;
use super::sinfo::SubjectInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let console = Console::init(SubjectInfo::init());
    let msg = info.message()
                  .as_str()
                  .unwrap_or("Unknown error");
    // We might not be able to write here due to console
    // being unable to initialise.
    console.map (|mut c| c.write(msg));
    loop {}
}
