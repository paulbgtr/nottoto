defmodule NottotoWeb.NotesController do
  use NottotoWeb, :controller

  alias Nottoto.Note
  alias Nottoto.Notes

  def index(conn, _params) do
    notes = Notes.list_notes()
    json(conn, notes)
  end

  def show(conn, %{"id" => id}) do
    note = Notes.get_note!(id)
    json(conn, note)
  end

  def create(conn, %{"note" => note_params}) do
    case Notes.create_note(note_params) do
      {:ok, note} ->
        conn
        |> put_status(:created)
        |> put_resp_header("location", Routes.note_path(conn, :show, note))
        |> render("show.json", note: note)

      {:error, changeset} ->
        conn
        |> put_status(:unprocessable_entity)
        |> render("error.json", changeset: changeset)
    end
  end
end
