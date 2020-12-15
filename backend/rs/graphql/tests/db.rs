use anyhow::Error;
use dotenv::dotenv;
use graphql::models::{
    Analysis, Dataset, Dataview, Model, Operation, Plot, PlotType, Project,
    Statistic, StatisticName, User,
};
use serde_json::Map as JMap;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio_compat_02::FutureExt;

#[tokio::test]
async fn db_round_trip() -> Result<(), Error> {
    // setup
    dotenv().ok();
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .compat()
        .await?;

    // user
    let user =
        User::create(&db, "Motoko Kusanagi", "motoko", "motoko@motoko.ai")
            .await?;

    // get user
    let same_user = User::get(&db, &user.uuid).await?;

    // eq
    assert_eq!(user, same_user);

    // project
    let project = Project::create(&db, "project 1", &user.uuid).await?;

    // dataset
    let dataset = Dataset::create(
        &db,
        &project.uuid,
        "dataset 1",
        "http://motoko.ai/test_dataset.csv",
    )
    .await?;

    // analysis
    let analysis = Analysis::create(&db, &dataset.uuid, "analysis 1").await?;

    // dataview transformation
    let mut op_args = JMap::new();
    op_args.insert("identifier".into(), "column_1".into());
    op_args.insert("comparator".into(), ">".into());
    op_args.insert("value".into(), 5.into());
    let dataview = Dataview::create(
        &db,
        &analysis.dataview_uuid,
        &Operation::Filter,
        &op_args.into(),
    )
    .await?;

    // analysis update
    Analysis::point_to(&db, &analysis.uuid, &dataview.uuid).await?;

    // statistic
    let mut stat_args = JMap::new();
    stat_args.insert("a".into(), "column_1".into());
    stat_args.insert("b".into(), "column_2".into());
    Statistic::create(
        &db,
        &dataview.uuid,
        &StatisticName::Correlation,
        &stat_args.into(),
    )
    .await?;
    let mut plot_args = JMap::new();

    // plot
    plot_args.insert("x".into(), "column_1".into());
    plot_args.insert("y".into(), "column_2".into());
    Plot::create(
        &db,
        &dataview.uuid,
        "plot 1",
        &PlotType::Scatter,
        &plot_args.into(),
    )
    .await?;

    // model
    let model = Model::create(
        &db,
        &dataview.uuid,
        "model 1",
        &Some("target".into()),
        &vec!["feature_1".into(), "feature_2".into(), "feature_3".into()],
        &serde_json::Value::Null,
    )
    .await?;

    // delete project (should cascade)
    Project::delete(&db, &project.uuid).await?;

    // confirm associated model has been deleted
    assert!(Model::get(&db, &model.uuid).await.is_err());

    // delete user
    User::delete(&db, &user.uuid).await?;

    Ok(())
}