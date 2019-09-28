FROM gitpod/workspace-postgres

USER gitpod
RUN bash -lc "cargo install diesel_cli"
