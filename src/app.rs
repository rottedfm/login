use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    // Current art frame
    pub art: String,

    // Queued art frames
    pub queue: Vec<String>,

    /// Current index in the queue
    current_frame: usize,

    /// Direction flag (true = forward, false = backward)
    forward: bool,

    /// Git log text
    pub git_log: Vec<String>,
    pub scroll: u16,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            art: String::new(),
            queue: vec![
                r#""#.to_string(),
                r#"
888 
888 
888 
888 
888 
888 
888 
888 
888 
888 
888 
                "#.to_string(),
                r#"
             888 
             888 
   .u    .   888 
 .d88B :@8c  888 
="8888f8888r 888 
  4888>'88"  888 
  4888> '    888 
  4888>      888 
 .d888L .+   888 
 ^"8888*"    888 
    "Y"      888 
                "#.to_string(),
                r#"
                         888 
                         888 
   .u    .          u.   888 
 .d88B :@8c   ...ue888b  888 
="8888f8888r  888R Y888r 888 
  4888>'88"   888R I888> 888 
  4888> '     888R I888> 888 
  4888>       888R I888> 888 
 .d888L .+   u8888cJ888  888 
 ^"8888*"     "*888*P"   888 
    "Y"         'Y"      888
                "#.to_string(),
                r#"
                              s    888 
                             :8    888 
   .u    .          u.      .88    888 
 .d88B :@8c   ...ue888b    :888ooo 888 
="8888f8888r  888R Y888r -*8888888 888 
  4888>'88"   888R I888>   8888    888 
  4888> '     888R I888>   8888    888 
  4888>       888R I888>   8888    888 
 .d888L .+   u8888cJ888   .8888Lu= 888 
 ^"8888*"     "*888*P"    ^%888*   888 
    "Y"         'Y"         'Y"    888 
                "#.to_string(),
                r#"
                              s         s    888 
                             :8        :8    888 
   .u    .          u.      .88       .88    888 
 .d88B :@8c   ...ue888b    :888ooo   :888ooo 888 
="8888f8888r  888R Y888r -*8888888 -*8888888 888 
  4888>'88"   888R I888>   8888      8888    888 
  4888> '     888R I888>   8888      8888    888 
  4888>       888R I888>   8888      8888    888 
 .d888L .+   u8888cJ888   .8888Lu=  .8888Lu= 888 
 ^"8888*"     "*888*P"    ^%888*    ^%888*   888 
    "Y"         'Y"         'Y"       'Y"    888
                "#.to_string(),
                r#"
                              s         s               888 
                             :8        :8               888 
   .u    .          u.      .88       .88               888 
 .d88B :@8c   ...ue888b    :888ooo   :888ooo      .u    888 
="8888f8888r  888R Y888r -*8888888 -*8888888   ud8888.  888 
  4888>'88"   888R I888>   8888      8888    :888'8888. 888 
  4888> '     888R I888>   8888      8888    d888 '88%" 888 
  4888>       888R I888>   8888      8888    8888.+"    888 
 .d888L .+   u8888cJ888   .8888Lu=  .8888Lu= 8888L      888 
 ^"8888*"     "*888*P"    ^%888*    ^%888*   '8888c. .+ 888 
    "Y"         'Y"         'Y"       'Y"     "88888%   888 
                                                "YP'       
                "#.to_string(),
                r#"
                              s         s                  ..       888 
                             :8        :8                dF         888 
   .u    .          u.      .88       .88               '88bu.      888 
 .d88B :@8c   ...ue888b    :888ooo   :888ooo      .u    '*88888bu   888 
="8888f8888r  888R Y888r -*8888888 -*8888888   ud8888.    ^"*8888N  888 
  4888>'88"   888R I888>   8888      8888    :888'8888.  beWE "888L 888 
  4888> '     888R I888>   8888      8888    d888 '88%"  888E  888E 888 
  4888>       888R I888>   8888      8888    8888.+"     888E  888E 888 
 .d888L .+   u8888cJ888   .8888Lu=  .8888Lu= 8888L       888E  888F 888 
 ^"8888*"     "*888*P"    ^%888*    ^%888*   '8888c. .+ .888N..888  888 
    "Y"         'Y"         'Y"       'Y"     "88888%    `"888*""   888 
                                                "YP'        ""         
                "#.to_string(),
                r#"
                              s         s                  ..               ...        888 
                             :8        :8                dF             .uW8***88e.    888 
   .u    .          u.      .88       .88               '88bu.         d8*"     `"8N.  888 
 .d88B :@8c   ...ue888b    :888ooo   :888ooo      .u    '*88888bu    .@8F   .ucu.. %8L 888 
="8888f8888r  888R Y888r -*8888888 -*8888888   ud8888.    ^"*8888N   @8E  .@8""988  8N 888 
  4888>'88"   888R I888>   8888      8888    :888'8888.  beWE "888L '88>  @8~  98F  98 888 
  4888> '     888R I888>   8888      8888    d888 '88%"  888E  888E ~88  X8E   98~  8E 888 
  4888>       888R I888>   8888      8888    8888.+"     888E  888E '88> 98&  d88  X8  888 
 .d888L .+   u8888cJ888   .8888Lu=  .8888Lu= 8888L       888E  888F  %8N '88W@"%8ed*`  888 
 ^"8888*"     "*888*P"    ^%888*    ^%888*   '8888c. .+ .888N..888    %8b. `"   ``     888 
    "Y"         'Y"         'Y"       'Y"     "88888%    `"888*""      `*8bu.. ..u@    888 
                                                "YP'        ""            ^"***%"`         
                "#.to_string(),
                r#"
                              s         s                  ..               ...                  888 
                             :8        :8                dF             .uW8***88e.       oec :  888 
   .u    .          u.      .88       .88               '88bu.         d8*"     `"8N.    @88888  888 
 .d88B :@8c   ...ue888b    :888ooo   :888ooo      .u    '*88888bu    .@8F   .ucu.. %8L   8"*88%  888 
="8888f8888r  888R Y888r -*8888888 -*8888888   ud8888.    ^"*8888N   @8E  .@8""988  8N   8b.     888 
  4888>'88"   888R I888>   8888      8888    :888'8888.  beWE "888L '88>  @8~  98F  98  u888888> 888 
  4888> '     888R I888>   8888      8888    d888 '88%"  888E  888E ~88  X8E   98~  8E   8888R   888 
  4888>       888R I888>   8888      8888    8888.+"     888E  888E '88> 98&  d88  X8    8888P   888 
 .d888L .+   u8888cJ888   .8888Lu=  .8888Lu= 8888L       888E  888F  %8N '88W@"%8ed*`    *888>   888 
 ^"8888*"     "*888*P"    ^%888*    ^%888*   '8888c. .+ .888N..888    %8b. `"   ``       4888    888 
    "Y"         'Y"         'Y"       'Y"     "88888%    `"888*""      `*8bu.. ..u@      '888    888 
                                                "YP'        ""            ^"***%"`        88R        
                                                                                          88>        
                                                                                          48         
                                                                                          '8         
                "#.to_string(),
                r#"``
                              s         s                  ..               ...                                      888 
                             :8        :8                dF             .uW8***88e.       oec :                      888 
   .u    .          u.      .88       .88               '88bu.         d8*"     `"8N.    @88888     ..    .     :    888 
 .d88B :@8c   ...ue888b    :888ooo   :888ooo      .u    '*88888bu    .@8F   .ucu.. %8L   8"*88%   .888: x888  x888.  888 
="8888f8888r  888R Y888r -*8888888 -*8888888   ud8888.    ^"*8888N   @8E  .@8""988  8N   8b.     ~`8888~'888X`?888f` 888 
  4888>'88"   888R I888>   8888      8888    :888'8888.  beWE "888L '88>  @8~  98F  98  u888888>   X888  888X '888>  888 
  4888> '     888R I888>   8888      8888    d888 '88%"  888E  888E ~88  X8E   98~  8E   8888R     X888  888X '888>  888 
  4888>       888R I888>   8888      8888    8888.+"     888E  888E '88> 98&  d88  X8    8888P     X888  888X '888>  888 
 .d888L .+   u8888cJ888   .8888Lu=  .8888Lu= 8888L       888E  888F  %8N '88W@"%8ed*`    *888>     X888  888X '888>  888 
 ^"8888*"     "*888*P"    ^%888*    ^%888*   '8888c. .+ .888N..888    %8b. `"   ``       4888     "*88%""*88" '888!` 888 
    "Y"         'Y"         'Y"       'Y"     "88888%    `"888*""      `*8bu.. ..u@      '888       `~    "    `"`   888 
                                                "YP'        ""            ^"***%"`        88R                            
                                                                                          88>                            
                                                                                          48                             
                                                                                          '8                             
                "#.to_string(),
            ],
            current_frame: 0,
            forward: true,
            git_log: vec![],
            scroll: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    // Fetch git log -p from ~/.dotfiles
    fn fetch_git_log(&mut self) {
        let dotfiles_path = 
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.cycle();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Move to the next frame in the queue
    pub fn cycle(&mut self) {
  if !self.queue.is_empty() {
            if self.forward {
                // Moving forward
                if self.current_frame == self.queue.len() - 1 {
                    // If we reach the last frame, switch direction to backward
                    self.forward = false;
                } else {
                    // Move forward to the next frame
                    self.current_frame += 1;
                }
            } else {
                // Moving backward
                if self.current_frame == 0 {
                    // If we reach the first frame, switch direction to forward
                    self.forward = true;
                } else {
                    // Move backward to the previous frame
                    self.current_frame -= 1;
                }
            }

            // Update the `art` to the current frame
            self.art = self.queue[self.current_frame].clone();
        }        
    }
}
