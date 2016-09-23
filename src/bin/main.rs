extern crate rgb_emu as rgb;

fn main () {
    let state = rgb::new();
    state.step();
}

///////////////////////////////////////////////////////////////////////////////
// SCRATCH-PAD
///////////////////////////////////////////////////////////////////////////////

// note: this was the old main function with the plan for the GTK interface.
//       the plan now is to have the main functionality in a library and then
//       extract the UI into it's own program.
//fn main() {
//
//    // Initalize the emulator hardware
//    let mut mmu:Rc<RefCell<_>> = Rc::new(RefCell::new(MMU::new()));
//    let mut cpu = Z80::new(mmu.clone());
//
//    // Initialize GTK
//    if gtk::init().is_err() {
//        println!("Failed to initialize GTK.");
//        return;
//    }
//
//    // In gtk all of the window objects are normally created before the window
//    // is shown to the user. Small windows are sometimes created during runtime
//    // but the idea is that any widget (or widget tree) which can draw itself
//    // can be instantiated and then hidden, only to be shown when it is needed.
//    //
//    // The main window here serves as a starting point of a new tree of GUI
//    // objects.
//    let window = Window::new(WindowType::Toplevel);
//
//    // We'll add our UI here.
//
//    // Connect / Register the UI events which we want to keep track of.
//    window.connect_delete_event(|_, _| {
//        gtk::main_quit();
//        Inhibit(false)
//    });
//
//    // Once we're ready to show the interface to the user we can use this
//    // (shortcut) function to show everything.
//    window.show_all();
//
//    // Give control of the main event loop to GTK
//    gtk::main();
//}

