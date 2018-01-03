#![windows_subsystem = "windows"]
#[macro_use]extern crate native_windows_gui as nwg;
#[derive(Debug, Clone, Hash)]
pub enum AppId {
    // Controls
    MainWindow,
    ItemCollection, 
    LoadButton,

    // Events
    LoadCollection,
    ItemSelected,

    // Resources
    TextFont
}

use AppId::*; // Shortcut
use std::process::Command;
// get list of R scripts in seascripts Package
fn get_scripts()-> Vec<String>{
			let a = Command::new("Rscript.exe")
        .args(&["-e", "cat(paste(list.files(system.file('scripts',package='seascripts')),collapse='\n'))"])
		.output()
        .expect("...!!!");
         let mydata: Vec<String> = String::from_utf8_lossy(&a.stdout).split_whitespace().map(|s| s.to_string()).collect();
		return mydata;
}
//launch R app
fn r_app_launch(input: &str){
let mut a = "source(system.file(package='seascripts',path='scripts/".to_string();
let ref b=input;
let c="'))".to_string();

let g = a + &b +&c;
println!("{}",&g);
Command::new("Rscript.exe")
        .args(&["-e",&g])
        .spawn()
        .expect("....!!!");
}

nwg_template!(
    head: setup_ui<AppId>,
    controls: [
	
	
        (MainWindow, nwg_window!( 
            title="seascripts Apps";
            size=(350, 75) 
            )),

        (ItemCollection, nwg_combobox!( 
             parent=MainWindow; 
             position=(10, 11); size=(200,45); 
             font=Some(TextFont);
             //collection=Vec::<String>::new();
			 collection = get_scripts();
             placeholder=Some("")  // Note: thanks to you, I just found out that this parameter must be 
            )),                    // present or else the macro fails... it will be fixed soon

        (LoadButton, nwg_button!( 
             parent=MainWindow; 
             text="Launch App"; 
             position=(240, 10); size=(85, 35); 
             font=Some(TextFont) 
            ))
    ];
    events: [

        (LoadButton, LoadCollection, nwg::Event::Click, |app,_,_,_| {
           let combobox = nwg_get_mut!(app; (ItemCollection, nwg::ComboBox<String>));
			let now = combobox.get_selected_text();
			//println!("{:?}",&now);
			r_app_launch(&now);

        })
    ];
    resources: [
        (TextFont, nwg_font!(family="Arial"; size=17))
    ];
    values: []
);

fn main() {
    let app = nwg::Ui::new().expect("Something went wrong...");
    setup_ui(&app).expect("Something went wrong...");
  
    // Uncomment this line if you want the data to be loaded automatically
     app.trigger(&LoadButton, nwg::Event::Click, nwg::EventArgs::None);
  
    nwg::dispatch_events();
}
