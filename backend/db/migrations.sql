create table
  public.users (
    id bigint generated by default as identity,
    email text not null,
    password text not null,
    created_at timestamp with time zone not null default now(),
    constraint users_pkey primary key (id),
    constraint users_email_key unique (email),
    constraint users_password_key unique (password)
  ) tablespace pg_default;

create table
  public.notes (
    id bigint generated by default as identity,
    title text not null,
    body text null,
    created_at timestamp with time zone not null default now(),
    user_id bigint null,
    constraint notes_pkey primary key (id),
    constraint notes_title_key unique (title),
    constraint notes_user_id_fkey foreign key (user_id) references users (id)
  ) tablespace pg_default;
