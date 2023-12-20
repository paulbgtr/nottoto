defmodule NottotoWeb.Router do
  use NottotoWeb, :router

  pipeline :api do
    plug(:accepts, ["json"])
  end

  scope "/api", NottotoWeb do
    pipe_through(:api)
    get("/", DefaultController, :index)
  end
end
