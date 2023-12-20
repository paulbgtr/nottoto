defmodule Nottoto.Notes do
  alias Nottoto.Repo
  alias Nottoto.Note

  def list_notes do
    Repo.all(Note)
  end

  def get_note!(id) do
    Repo.get!(Note, id)
  end

  def create_note(attrs \\ %{}) do
    %Note{}
    |> Note.changeset(attrs)
    |> Repo.insert()
  end
end
