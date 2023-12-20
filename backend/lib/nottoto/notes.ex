defmodule Nottoto.Notes do
  alias Nottoto.Repo
  alias Nottoto.Note

  def list_notes do
    Repo.all(Note)
  end

  def get_post!(id) do
    Repo.get!(Note, id)
  end
end
