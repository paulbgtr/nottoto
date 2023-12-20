defmodule NottotoWeb.DefaultController do
  use NottotoWeb, :controller

  def index(conn, _params) do
    text(conn, "Hello World")
  end
end
