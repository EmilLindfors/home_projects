
create table task
(
    task_id  uuid primary key     default uuid_generate_v1mc(),
    description TEXT    NOT NULL,
    done        BOOLEAN NOT NULL DEFAULT FALSE
);

create table project_tasks
(
    project_id uuid        not null references project (project_id) on delete cascade,
    task_id    uuid        not null references task (task_id) on delete cascade,

    created_at timestamptz not null default now(),
    updated_at timestamptz,

    -- Enforce uniqueness like with `follow`.
    primary key (project_id, task_id)

    -- Unlike with follows, it's more than a simple check constraint to forbid an author from favoriting their own
    -- project. Since the Realworld spec doesn't say either way, and it's foreseeable that an author might
    -- want to favorite some of their own projects like "these are my best works", we'll allow it.
);

select trigger_updated_at('project_tasks');