mod client_connection;
mod window_handler;

use self::{client_connection::ClientConnection, window_handler::WindowHandler};

use super::lua_engine::LuaEngine;

///
/// The Client component for the engine.
///
/// The Client component has 5 jobs:
/// 1.) Hold a window.
/// 2.) Hold the render engine.
/// 3.) Hold a ClientConnection which handles talking to a server.
/// * 4.) [in the future] Be the main handler for ClientAuthentication.
/// *  - ClientAuthentication does exactly what you think it does.
/// *  - Maintains a client auth for itself when talking to the server.
/// ? 5.) Handle GameConfig as a component. This should be received from a server
/// ? 5 - Marked with ? because it's still being thought out at the moment.
///
pub struct Client {
  window_handler: WindowHandler,
  client_name: String,
  connection: Option<ClientConnection>,
  lua_engine: Option<LuaEngine>,
}

impl Client {
  pub fn new(client_name: String, address: String, port: i32) -> Self {
    let mut new_client = Client {
      window_handler: WindowHandler::new(),
      client_name,
      connection: None, //ClientConnection::new(address, port),
      lua_engine: None,
    };

    new_client.reset_lua_vm();

    println!("minetest: player name is: {}", &new_client.client_name);

    new_client
  }

  ///
  /// Change the client's name.
  ///
  pub fn change_name(&mut self, new_client_name: String) {
    self.client_name = new_client_name;
  }

  ///
  /// Get the client's name.
  ///
  pub fn get_name(&self) -> String {
    // Just fire off new heap memory.
    self.client_name.clone()
  }

  ///
  /// Deletes the lua VM.
  ///
  fn delete_lua_vm(&mut self) {
    self.lua_engine = None
  }

  ///
  /// Creates a new client lua VM.
  ///
  fn create_lua_vm(&mut self) {
    self.lua_engine = Some(LuaEngine::new(false));
  }

  ///
  /// Wipe the memory of the lua VM.
  /// Automatically regenerates a blank client VM.
  ///
  pub fn reset_lua_vm(&mut self) {
    self.delete_lua_vm();
    self.create_lua_vm();
  }

  ///
  /// Tick tock.
  ///
  /// Every time the game goes into the next main loop iteration
  /// this is run.
  ///
  /// This is referred to as on_step in C++ minetest.
  ///
  pub fn on_tick(&mut self, delta: f64) {
    // Poll any incoming network traffic. (non blocking)

    match &mut self.connection {
      Some(connection) => connection.receive(delta),
      None => (),
    }

    // We want this to throw a runtime panic if we make a logic error.
    // ! Never turn this into a silent bypass via: is_some()
    match &self.lua_engine {
      Some(lua_engine) => lua_engine.on_tick(delta),
      None => panic!("minetest: Client LuaEngine does not exist!"),
    }
  }
}
