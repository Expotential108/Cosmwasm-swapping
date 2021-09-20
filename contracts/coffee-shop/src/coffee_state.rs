use crate::products::{CoffeeCup, IngredientPortion};
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CoffeeState {
    pub menu: Vec<CoffeeCup>,
    pub ingredient_portions: Vec<IngredientPortion>,
}

// &[u8]
pub const COFFEE_STATE: Map<String, CoffeeState> = Map::new("coffee_state");