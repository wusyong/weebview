use wry::{Application, Attributes, Result};

static TEST_HTML: &str = r#"data:text/html,
Drop files onto the window and read the console!<br>
Dropping files onto the following form is also possible:<br><br>
<input type="file"/>
"#;

fn main() -> Result<()> {
  let mut app = Application::new()?;

  app.add_window_with_configs(
    Attributes {
      url: Some(TEST_HTML.to_string()),
      ..Default::default()
    },
    None,
    None,
    Some(Box::new(|data| {
      println!("Window 1: {:?}", data);
      false // Returning true will block the OS default behaviour.
    })),
  )?;
  app.run();
  Ok(())
}
