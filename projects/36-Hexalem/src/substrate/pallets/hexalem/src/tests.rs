use crate::{
	mock::{self, *},
	pallet, Error, Event, GameProperties, GameState, GameStorage, GetTileInfo, HexBoard,
	HexBoardStorage, HexGrid, Move, ResourceType, NUMBER_OF_RESOURCE_TYPES,
};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_new_game_successfully() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.

		let players = vec![1, 2, 3];

		assert_ok!(HexalemModule::create_game(RuntimeOrigin::signed(1), players.clone(), 25));
		// Read pallet storage and assert an expected result.
		let hex_board_option: Option<crate::HexBoard<TestRuntime>> =
			HexBoardStorage::<TestRuntime>::get(1);

		let hex_board = hex_board_option.unwrap();

		assert_eq!(
			hex_board.resources,
			<mock::TestRuntime as pallet::Config>::DefaultPlayerResources::get()
		);

		let default_hex_grid: HexGrid<TestRuntime> = vec![
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile::get_home(),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
		]
		.try_into()
		.unwrap();
		assert_eq!(hex_board.hex_grid, default_hex_grid);

		let game_id = hex_board.game_id;

		// Assert that the correct event was deposited
		System::assert_last_event(
			Event::GameCreated { game_id, grid_size: 25, players: players.clone() }.into(),
		);

		let game_option = GameStorage::<TestRuntime>::get(game_id);

		let game = game_option.unwrap();

		assert_eq!(game.players, players.clone());

		assert_eq!(game.get_player_turn(), 0);

		assert_eq!(game.get_played(), false);

		assert_eq!(game.get_round(), 0);

		assert_eq!(game.get_selection_size(), 2);

		assert_eq!(game.get_state(), GameState::Playing);

		let current_selection_indexes = game.selection.clone();

		let selection_one_cost = <mock::TestRuntime as pallet::Config>::TileCosts::get()
			[current_selection_indexes[0] as usize];

		let move_played = Move { place_index: 11, buy_index: 0 };

		assert_eq!(selection_one_cost.cost.resource_type, ResourceType::Mana);
		assert_eq!(selection_one_cost.cost.amount, 1);

		assert_ok!(HexalemModule::play(RuntimeOrigin::signed(1), move_played.clone()));

		System::assert_last_event(Event::MovePlayed { game_id, player: 1, move_played }.into());

		let hex_board_option: Option<crate::HexBoard<TestRuntime>> =
			HexBoardStorage::<TestRuntime>::get(1);

		let hex_board = hex_board_option.unwrap();
		assert_eq!(hex_board.resources, [0, 1, 0, 0, 0, 0, 0]);

		let expected_hex_grid: HexGrid<TestRuntime> = vec![
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			selection_one_cost.tile_to_buy,
			HexalemTile::get_home(),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
		]
		.try_into()
		.unwrap();
		assert_eq!(hex_board.hex_grid, expected_hex_grid);

		let game_option = GameStorage::<TestRuntime>::get(game_id);

		let game = game_option.unwrap();

		assert_eq!(game.players, players.clone());

		assert_eq!(game.get_player_turn(), 0);

		assert_eq!(game.get_played(), true);

		assert_eq!(game.get_round(), 0);

		assert_eq!(game.get_selection_size(), 4);

		assert_eq!(game.get_state(), GameState::Playing);

		assert_ok!(HexalemModule::finish_turn(RuntimeOrigin::signed(1)));

		System::assert_last_event(Event::NewTurn { game_id, next_player: 2 }.into());

		let hex_board_option: Option<crate::HexBoard<TestRuntime>> =
			HexBoardStorage::<TestRuntime>::get(1);

		let hex_board = hex_board_option.unwrap();
		assert_eq!(hex_board.resources, [1, 1, 2, 0, 0, 0, 0]);

		assert_eq!(hex_board.hex_grid, expected_hex_grid);

		let game_option = GameStorage::<TestRuntime>::get(game_id);

		let game = game_option.unwrap();

		assert_eq!(game.players, players.clone());

		assert_eq!(game.get_player_turn(), 1);

		assert_eq!(game.get_played(), false);

		assert_eq!(game.get_round(), 0);

		assert_eq!(game.get_selection_size(), 4);

		assert_eq!(game.get_state(), GameState::Playing);
	});
}

#[test]
fn create_new_game_fails_number_of_players_is_too_small() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			HexalemModule::create_game(RuntimeOrigin::signed(1), vec![], 25),
			Error::<TestRuntime>::NumberOfPlayersIsTooSmall
		);
	});
}

#[test]
fn create_new_game_fails_bad_grid_size() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			HexalemModule::create_game(RuntimeOrigin::signed(1), vec![1], 1),
			Error::<TestRuntime>::BadGridSize
		);
	});

	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			HexalemModule::create_game(RuntimeOrigin::signed(1), vec![1], 2),
			Error::<TestRuntime>::BadGridSize
		);
	});
}

#[test]
fn test_resource_generation() {
	new_test_ext().execute_with(|| {
		assert_ok!(HexalemModule::create_game(RuntimeOrigin::signed(1), vec![1], 25));

		let new_hex_grid: HexGrid<TestRuntime> = vec![
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(56),
			HexalemTile(48),
			HexalemTile(40),
			HexalemTile(32),
			HexalemTile(24),
			HexalemTile(16),
			HexalemTile(8),
			HexalemTile::get_home(),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
		]
		.try_into()
		.unwrap();

		let hex_board_option: Option<HexBoard<TestRuntime>> =
			HexBoardStorage::<TestRuntime>::get(1);

		let hex_board = hex_board_option.unwrap();

		let game_id = hex_board.game_id;

		HexalemModule::set_hex_board(
			1,
			HexBoard { game_id, hex_grid: new_hex_grid, resources: [0, 1, 0, 0, 0, 0, 0] },
		);

		assert_ok!(HexalemModule::finish_turn(RuntimeOrigin::signed(1)));

		let hex_board_option: Option<HexBoard<TestRuntime>> =
			HexBoardStorage::<TestRuntime>::get(1);

		let hex_board = hex_board_option.unwrap();

		assert_eq!(hex_board.resources, [2, 2, 2, 3, 1, 1, 1]);
	});
}

#[test]
fn test_saturate_99() {
	new_test_ext().execute_with(|| {
		assert_ok!(HexalemModule::create_game(RuntimeOrigin::signed(1), vec![1], 25));

		let new_hex_grid: HexGrid<TestRuntime> = vec![
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(56),
			HexalemTile(48),
			HexalemTile(40),
			HexalemTile(32),
			HexalemTile(24),
			HexalemTile(16),
			HexalemTile(8),
			HexalemTile::get_home(),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
			HexalemTile(0),
		]
		.try_into()
		.unwrap();

		let hex_board_option: Option<HexBoard<TestRuntime>> =
			HexBoardStorage::<TestRuntime>::get(1);

		let hex_board = hex_board_option.unwrap();

		let game_id = hex_board.game_id;

		// Set player resources to 99 and set a new hex_grid
		HexalemModule::set_hex_board(
			1,
			HexBoard { game_id, hex_grid: new_hex_grid, resources: [99; NUMBER_OF_RESOURCE_TYPES] },
		);

		assert_ok!(HexalemModule::finish_turn(RuntimeOrigin::signed(1)));

		let hex_board_option: Option<HexBoard<TestRuntime>> =
			HexBoardStorage::<TestRuntime>::get(1);

		let hex_board = hex_board_option.unwrap();

		assert_eq!(hex_board.resources, [99, 6, 99, 99, 99, 99, 99]);
	});
}

#[test]
fn test_force_finish_turn() {
	new_test_ext().execute_with(|| {
		assert_ok!(HexalemModule::create_game(RuntimeOrigin::signed(1), vec![1, 2], 25));

		let hex_board_option: Option<HexBoard<TestRuntime>> =
			HexBoardStorage::<TestRuntime>::get(1);

		let hex_board = hex_board_option.unwrap();

		let game_id = hex_board.game_id;

		// force_finish_turn can not be called before the BlocksToPlayLimit has been passed
		assert_noop!(
			HexalemModule::force_finish_turn(RuntimeOrigin::signed(2), game_id),
			Error::<TestRuntime>::BlocksToPlayLimitNotPassed
		);

		System::set_block_number(
			<mock::TestRuntime as pallet::Config>::BlocksToPlayLimit::get() as u64 + 1,
		);

		// force_finish_turn can not be called by the player that is currently on turn
		assert_noop!(
			HexalemModule::force_finish_turn(RuntimeOrigin::signed(1), game_id),
			Error::<TestRuntime>::CurrentPlayerCannotForceFinishTurn
		);

		// force_finish_turn can not be called by the player that is not in the game
		assert_noop!(
			HexalemModule::force_finish_turn(RuntimeOrigin::signed(3), game_id),
			Error::<TestRuntime>::PlayerNotInGame
		);

		// Now that enough blocks have passed, force_finish_turn can be called
		assert_ok!(HexalemModule::force_finish_turn(RuntimeOrigin::signed(2), game_id));
	})
}

#[test]
fn play() {
	new_test_ext().execute_with(|| {
		assert_ok!(HexalemModule::create_game(RuntimeOrigin::signed(1), vec![1, 2], 25));

		let hex_board_option: Option<HexBoard<TestRuntime>> =
			HexBoardStorage::<TestRuntime>::get(1);

		let hex_board = hex_board_option.unwrap();

		let game_id = hex_board.game_id;

		
		assert_noop!(
			HexalemModule::play(RuntimeOrigin::signed(1), Move {place_index: 12, buy_index: 0}),
			Error::<TestRuntime>::TileIsNotEmpty
		);

		// newly placed tile needs to connect to already placed tiles
		assert_noop!(
			HexalemModule::play(RuntimeOrigin::signed(1), Move {place_index: 0, buy_index: 0}),
			Error::<TestRuntime>::TileSurroundedByEmptyTiles
		);

		assert_noop!(
			HexalemModule::play(RuntimeOrigin::signed(1), Move {place_index: 26, buy_index: 0}),
			Error::<TestRuntime>::PlaceIndexOutOfBounds
		);

		assert_noop!(
			HexalemModule::play(RuntimeOrigin::signed(1), Move {place_index: 11, buy_index: 2}),
			Error::<TestRuntime>::BuyIndexOutOfBounds
		);

		// Set player resources to 0
		HexalemModule::set_hex_board(
			1,
			HexBoard {
				game_id,
				hex_grid: hex_board.hex_grid,
				resources: [0; NUMBER_OF_RESOURCE_TYPES],
			},
		);

		assert_noop!(
			HexalemModule::play(RuntimeOrigin::signed(1), Move {place_index: 11, buy_index: 0}),
			Error::<TestRuntime>::NotEnoughResources
		);

		assert_noop!(
			HexalemModule::play(RuntimeOrigin::signed(2), Move {place_index: 11, buy_index: 0}),
			Error::<TestRuntime>::PlayerNotOnTurn
		);

		assert_noop!(
			HexalemModule::play(RuntimeOrigin::signed(3), Move {place_index: 11, buy_index: 0}),
			Error::<TestRuntime>::HexBoardNotInitialized
		);

		
	})
}
