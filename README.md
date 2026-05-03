# plate-spinner
App for staying on top of recurring tasks

```                                                                          
                                                  ░     ░░                                 
                      ░░▓▒                ░░▒░░░ ░▒▓▓▓▒░  ░░                               
                 ░░ ░░                   ░░░ ░ ▒▒▒▓▓▓▓░                        ░░          
              ░▒░░▓▓▓░                           ░ ░                  ░░░▓░ ░░   ░         
           ░▓░▒▓▒▓▓░░                              ▓               ░▓░░░▓▓▓▓▓▒░  ░▒ ░      
         ░░ ░      ▒    ░▓░▓░                      ░░             ▒   ░▓▓▓▓        ░░░░    
                    ░   ░▒░▓░░░░                     ░                    ░░      ░▒▓▓░░   
                    ░     ░▒▒▓▓░ ▓░                   ░                    ▒       ░░▒░  ░ 
                     ▓    ░░░     ░                 ░░█▒░░                  ░     ▒     ░░ 
                     ░░    ▒                      ░░██████░                ░░   ░█░        
                       ░  ░                    ░ ░▒███████                     ░▓          
                       ░ ░                ░░░░█▓  ▓██████                   ░░░▒           
                      ░ ░▒ ░         ░░░▒███████▓░████████░░               ░░▓▒            
                      ░██████████░░█████████████░▓░████████████░  ░░░███████▒██░           
                       ░░░  ░░█████████████████ ▓░░████████████████████░░  ░▒              
           ░ ░            █         ░   ░█████░▓▓░█████       ░ ░             ▓            
   ░░▓░░░▓▓ ░░░           ░            ░█████░▓▓░██████                       ░            
░░░░▓▓▓▓░░░                           ░█████ ░▓░░██████                                    
      ░                              ░█████  ░  ██████▒░                                   
       ░░                    ░░░░█████████████████████░                                    
        ░               ░░████████████████████████████                                     
        ░             ░▒████████████████▓░ ░███████░                                       
        ░░          ░█████████▒░              ██████░                                      
         █       ░░█████▓                       █████▒░                                    
         ░     ░░████░                            █████░                                   
          ░  ░████░░                             ░████                                     
          ▒░██▒                                 ▓████░                                     
        ░███░                                 ░████░                                       
                                            ░░███                                          
                                            ░██▒                                           
                                            ██                                             
                         ░░░░░░░░░░░░░░░░░░░███░░░░░░░░░░░░░░░░░░ ░                        
                                ░    ░  ░░         ░   ░  ░              
```

## Data Schema

- Plate
 - id: auto number
 - title: varchar
 - description: varchar
 - frequency: interval
 - next: date
 - started: date
 - saved: uint
 - spinning: bool - deleted
- Stats
 - saves: uint
 - drops: uint
 - ???
- Frequency
 - spins this week
 - drops this week

## Features

- [ ] Config file for
 - [ ] DB URI
 - [ ] ??
- [ ] CRUD
 - [ ] Add plate
 - [ ] Edit plate
 - [ ] Pause plate
 - [ ] export plates
- [ ] list next X plates to drop
- [ ] spin plate X
- [ ] list all plates
- [ ] list all paused plates

## Useful tools

- Image to ASCII - https://www.asciiart.eu/image-to-ascii

