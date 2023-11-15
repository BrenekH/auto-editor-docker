FROM python:3.12-slim-bookworm

COPY install-packages.sh .
RUN ./install-packages.sh

COPY run.sh .

RUN python -m pip install auto-editor yt-dlp

ENTRYPOINT [ "/run.sh" ]
