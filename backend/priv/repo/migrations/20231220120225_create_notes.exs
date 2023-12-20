defmodule Nottoto.Repo.Migrations.CreateNotes do
  use Ecto.Migration

  def change do
    create table(:notes, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :title, :text
      add :body, :text
      add :created_at, :utc_datetime

      timestamps(type: :utc_datetime)
    end
  end
end
