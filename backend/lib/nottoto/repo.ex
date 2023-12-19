defmodule Nottoto.Repo do
  use Ecto.Repo,
    otp_app: :nottoto,
    adapter: Ecto.Adapters.Postgres
end
