/* TODO
- [ ] basic TUI app
- [ ] config for DB location
- [ ] Control panel
- [ ] Add plate
- [ ] Edit plate
- [ ] Listing - with list option
- [ ] Select item from list to spin (or unpause if looking at paused plates)
- [ ] About and other polish
*/

use ps_core::plate_data::{connect,Action,List,DBError};


fn main() {
    println!("Hello, world!");
    
    let test = connect("memory");
    
    assert!(test.is_ok());
}


