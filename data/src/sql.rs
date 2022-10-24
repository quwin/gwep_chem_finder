use sqlx::ConnectOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use std::str::FromStr;
use crate::chemicals::{Reaction, BASES_MAP, Recipe, RawReagent};

#[tokio::main]
pub async fn get_reactions() -> Result<Vec<Reaction>, sqlx::Error > {
    dotenvy::dotenv().ok();

    let env = &std::env::var("DATABASE_URL").ok().unwrap();

    let mut reactions: Vec<Reaction> = Vec::new();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .journal_mode(SqliteJournalMode::Wal)
        .connect().await?;

    let num_of_reactions = sqlx::query!(
        "
        SELECT recipes
        FROM reactions
        ORDER BY recipes DESC;
        "
    )
    .fetch_one(&mut conn)
    .await?.recipes;

    for num in 0..num_of_reactions {
        let recipes = sqlx::query!(
            r#"
            SELECT *
            FROM recipes
            WHERE reaction = ?;
            "#,
            num
        )
        .fetch_all(&mut conn)
        .await?;

        let mut recipe_list: Vec<Recipe> = Vec::new();

        for recipe in recipes {
            let unwraped = recipe.reagents.unwrap();
            let reagents = sqlx::query!(
                r#"
                SELECT *
                FROM reagents
                WHERE recipe = ?;
                "#,
                unwraped
            )
            .fetch_all(&mut conn)
            .await?;
            let mut recipes_reagents: Vec<RawReagent> = Vec::new();
            for reagent in reagents {
                recipes_reagents.push(
                    RawReagent { 
                        name: reagent.name, 
                        quantity: reagent.amount as u32 
                    }
                )
            }
            let struc = Recipe::new(
                recipe.id,
                recipes_reagents,
                recipe.result_amount as f32, 
            );
            recipe_list.push(struc);
        }
        let reaction_query = sqlx::query!(
            r#"
            SELECT *
            FROM reactions
            WHERE recipes = ?;
            "#,
            num
        )
        .fetch_one(&mut conn)
        .await?;

        let required_temp: Option<f32>;

        match reaction_query.required_temp {
            Some(temp) => { required_temp = Some(temp as f32)}
            _ => { required_temp = None }
        }

        let reaction = Reaction::new(
            reaction_query.internal_name.unwrap(),
            reaction_query.name,
            reaction_query.result,
            recipe_list,
            reaction_query.mix_phrase,
            required_temp,
            reaction_query.instant,
            reaction_query.hidden
        );

        reactions.push(reaction);
    }
    Ok(reactions)
}

#[tokio::main]
pub async fn add_reaction(reactions: Vec<Reaction>) -> Result<(), sqlx::Error > {
    dotenvy::dotenv().ok();

    let env = &std::env::var("DATABASE_URL").ok().unwrap();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .journal_mode(SqliteJournalMode::Wal)
        .connect().await?;

    let mut reaction_list: Vec<String> = Vec::new();

    let mut first_counter: i32 = 0;
    for index in 0..reactions.len() {
        let reaction = reactions[index].clone();
        let internal_name = reaction.get_internal_name();
        reaction_list.push(internal_name.clone());
        let name = reaction.get_name();
        let result = reaction.get_result();
        let mix_phrase = reaction.get_mix_phrase();
        let instant = reaction.is_instant();
        let hidden = reaction.is_hidden();
        let placed_index = index as i32;

        sqlx::query!(
            r#"INSERT INTO reactions
            (internal_name, name, result, mix_phrase, instant, hidden, recipes)
            VALUES (?,?,?,?,?,?,?);
            "#,
            internal_name,
            name,
            result,
            mix_phrase,
            instant,
            hidden,
            placed_index
            )
            .execute(&mut conn)
            .await?;

        if let Some(temp) = reaction.get_required_temp() {
            sqlx::query!(
                r#"UPDATE reactions
                SET required_temp = ?
                WHERE internal_name = ?
                "#,
                temp,
                internal_name,
                )
                .execute(&mut conn)
                .await?;
        }
        for num in 0..reaction.recipe_amount() {
            let reaction_id = index as i32;
            let recipe_index = num as i32;
            let id = reaction.get_id_of_recipe(num);
            let result_amount = reaction.get_specific_recipe_result_amount(num);
            sqlx::query!(
                r#"INSERT INTO recipes
                (reaction, recipe_index, id, reagents, result_amount)
                VALUES (?,?,?,?,?);
                "#,
                reaction_id,
                recipe_index,
                id,
                first_counter,
                result_amount,
                )
                .execute(&mut conn)
                .await?;
            
        first_counter += 1;
        }
    }
    let mut second_counter: i32 = 0;
    for index in 0..reactions.len() {
        let reaction = reactions[index].clone();
        for num in 0..reaction.recipe_amount() {
            for reagent in reaction.get_reagents_of_recipe(num) {
                let name = reagent.name.clone();
                let amount = reagent.quantity;
                sqlx::query!(
                    r#"INSERT INTO reagents
                    (recipe, name, amount)
                    VALUES (?,?,?);
                    "#,
                    second_counter,
                    name,
                    amount
                    )
                    
                    .execute(&mut conn)
                    .await?;
                if reaction_list.contains(&name) {
                    let name = reagent.name.clone();
                    sqlx::query!(
                        r#"UPDATE reagents
                        SET ingredient_type = 'chemical'
                        WHERE name LIKE ? AND recipe = ?
                        "#,
                        name,
                        second_counter
                        )
                        .execute(&mut conn)
                        .await?;
                } else if BASES_MAP.contains_key(name.as_str()) {
                    let name = reagent.name.clone();
                    sqlx::query!(
                        r#"UPDATE reagents
                        SET ingredient_type = 'base'
                        WHERE name LIKE ? AND recipe = ?
                        "#,
                        name,
                        second_counter
                        )
                        .execute(&mut conn)
                        .await?;
                } else {
                }
            }
            second_counter += 1;
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn add_reactions(result: Result<(), sqlx::Error >) { 
    println!("{:?}", result)
}


#[tokio::main]
pub async fn database() -> Result<(), sqlx::Error > {
    dotenvy::dotenv().ok();

    let env = &std::env::var("DATABASE_URL").ok().unwrap();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .connect().await?;

    sqlx::query(
        "
        DROP TABLE IF EXISTS reagents;
        DROP TABLE IF EXISTS recipes;
        DROP TABLE IF EXISTS reactions;
        "
        )
        .execute(&mut conn)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS reactions (
                internal_name TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                result TEXT NOT NULL,
                mix_phrase TEXT NOT NULL,
                required_temp FLOAT,
                instant BOOLEAN NOT NULL,
                hidden BOOLEAN NOT NULL,
                recipes INT NOT NULL UNIQUE
            );"
        )
        .execute(&mut conn)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS recipes (
                reaction INT,
                recipe_index INT NOT NULL,
                id TEXT NOT NULL,
                reagents INT PRIMARY KEY,
                result_amount FLOAT NOT NULL,
                FOREIGN KEY(reaction) REFERENCES reactions(recipes)
            );"
        )
        .execute(&mut conn)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS reagents (
                recipe INT,
                name TEXT NOT NULL,
                ingredient_type TEXT NOT NULL DEFAULT 'ingredient',
                amount INT NOT NULL,
                FOREIGN KEY(recipe) REFERENCES recipes(reagents)
            );"
        )
        .execute(&mut conn)
        .await?;

    Ok(())
}

#[tokio::main]
pub async fn setup_database(result: Result<(), sqlx::Error >) { 
    println!("{:?}", result)
}
