# TIL TFX

[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black)

## Done

- `curl -sSL https://raw.githubusercontent.com/sdispater/poetry/master/get-poetry.py | python`
- `poetry init`
- `poetry config settings.virtualenvs.in-project true`
- `poetry add tensorflow==2.1.0-rc0 tfx==0.15.0`
- `poetry add -D black --allow-prereleases`
- `poetry add -D pylint pytest bandit isort autoflake`

## Pycharm

- Install
- Open project
- Setting Preferences
    - GitHub
- Set Sources to `til_tfx`
- Set Excludes to `notebooks` and `.` directories 
- Set External Tools for Black
    - https://black.readthedocs.io/en/stable/editor_integration.html
- Install Plugins
    - FileWathcer
    - Toml
    - Cloud Code
- Set Black to FileWatcher
    - https://black.readthedocs.io/en/stable/editor_integration.html
- Check `Preferences > Editor > General > Ensure line feed at file end on Save`
