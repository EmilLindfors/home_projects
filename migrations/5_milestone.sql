create table milestone
(
    milestone_id  uuid primary key     default uuid_generate_v1mc(),

    project_id     uuid        not null references "project" (project_id) on delete cascade,

    -- An project slug appears to be the title stripped down to words separated by hyphens.
    --
    -- As it is used to look up an project, it must be unique.
    --
    -- If we wanted to codify the slugification functionality in Postgres, we could make this a generated column,
    -- but it makes more sense to me to do that in Rust code so it can be unit tested.

    title       text        not null,

    description text        not null,
    body        text        not null,
    start_date  date not null default now(),
    end_date date


    -- These fields are actually in the Realworld spec so we will be making use of them.
    created_at  timestamptz not null default now(),

    -- The Realworld spec requires this to always be set,
    -- but we prefer to leave it null unless the row has actually been updated.
    -- It saves space as well as informs us whether a row has ever been updated or not.
    updated_at timestamptz not null default now()
);

select trigger_updated_at('milestone');

