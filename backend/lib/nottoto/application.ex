defmodule Nottoto.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      NottotoWeb.Telemetry,
      Nottoto.Repo,
      {DNSCluster, query: Application.get_env(:nottoto, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Nottoto.PubSub},
      # Start a worker by calling: Nottoto.Worker.start_link(arg)
      # {Nottoto.Worker, arg},
      # Start to serve requests, typically the last entry
      NottotoWeb.Endpoint
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Nottoto.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    NottotoWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
