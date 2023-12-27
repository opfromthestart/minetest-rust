mod server_connection;

use std::{cell::RefCell, ops::Deref, rc::Rc};

use self::server_connection::ServerConnection;

use super::{lua_engine::LuaEngine, Game};

pub struct Server<'server> {
  lua_engine: Option<LuaEngine<'server>>,
  connection: Option<ServerConnection<'server>>,
  game_pointer: Rc<RefCell<Game<'server>>>,
  server_pointer: Option<Rc<RefCell<Server<'server>>>>,
}

impl<'server> Server<'server> {
  pub fn new(
    game_pointer: Rc<RefCell<Game<'server>>>,
    address: String,
    port: i32,
  ) -> Rc<RefCell<Self>> {
    let new_server = Rc::new(RefCell::new(Server {
      lua_engine: None,
      connection: None,
      game_pointer: game_pointer.clone(),
      server_pointer: None,
    }));

    // The Server component will live for the lifetime of the program.
    // We need the ability for it to pass the reference to itself outwards.
    new_server.deref().borrow_mut().server_pointer = Some(new_server.clone());

    // Create the actual ServerConnection component.
    // This is utilized to actually talk to the clients that are connected.
    new_server.deref().borrow_mut().connection =
      Some(ServerConnection::new(new_server.clone(), address, port));

    // Automatically create a new Server LuaEngine.
    new_server.deref().borrow_mut().reset_lua_vm();

    new_server
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
    self.lua_engine = Some(LuaEngine::new(self.game_pointer.clone(), true));
  }

  ///
  /// Wipe the memory of the lua VM.
  /// Automatically regenerates a blank server VM.
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
    // Process any incoming network traffic. (non blocking)
    match &mut self.connection {
      Some(connection) => {
        connection.receive();
      }
      None => panic!("minetest: tried to receive data on a non-existent Server connection!"),
    }

    // We want this to throw a runtime panic if we make a logic error.
    // ! Never turn this into a silent bypass via: is_some()
    match &self.lua_engine {
      Some(lua_engine) => lua_engine.on_tick(delta),
      None => panic!("minetest: Server LuaEngine does not exist!"),
    }
  }
}
