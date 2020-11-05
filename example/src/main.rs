use browser_window::*;
use std::process::exit;
use tokio;



fn main() {

	let app = Application::start();

	// Start the tokio runtime and run our program logic on it
	let runtime = tokio::runtime::Runtime::new().unwrap();
	runtime.spawn( program_logic( app.clone().into() ) );

	let exit_code = app.run();

	// Return exit code
	exit( exit_code );
}

async fn program_logic( app: ApplicationAsync ) {

	let x = {
		let bw = BrowserWindowBuilder::new( Source::Html( include_str!("example.html").into() ) )
		.title("Example")
		.width( 800 )
		.height( 600 )
		.minimizable( false )
		.maximizable( false )
		.borders( false )
		.resizable( false )
		.handler(|_, cmd, args| {

			println!("Command \"{}\" invoked!", cmd);
			for i in 0..args.len() {
				println!("\tArg {}: {}", i+1, args[i]);
			}
		})
		.spawn_async( &app ).await;

		let bw2 = BrowserWindowBuilder::new( Source::Html( include_str!("example.html").into() ) )
			.title("Example")
			.width( 800 )
			.height( 600 )
			.minimizable( false )
			.maximizable( false )
			.borders( false )
			.resizable( true )
			.parent( &bw )
			.spawn_async( &app ).await;

		// Let's fetch the title through Javascript
		match bw.eval_js("document.title").await {
			Err(e) => { eprintln!("Something went wrong with evaluating javascript: {}", e) },
			Ok( cookies ) => {
				eprintln!("This is the window title: {}", cookies);
			}
		}

		// Let's execute some bad code
		// This doesn't work because cookies are not available when using Source::Html.
		match bw.eval_js("document.cookie").await {
			Err(e) => { eprintln!("This javascript error is expected when using CEF: {}", e) },
			Ok( cookies ) => {
				eprintln!("Available cookies: {}", cookies);
			}
		}

		bw2
	};

	tokio::time::delay_for( tokio::time::Duration::from_millis(30000) ).await;

	match x.eval_js("document.cookie").await {
		Err(e) => { eprintln!("This javascript error is expected when using CEF: {}", e) },
		Ok( cookies ) => {
			eprintln!("Available cookies: {}", cookies);
		}
	}
}
