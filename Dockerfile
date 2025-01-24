## Build
FROM soerenmeier/chuchi-build

COPY --chown=build . .

# then build the ui
WORKDIR /home/build/ui

RUN npm ci
RUN npm run build

# now build the server
WORKDIR /home/build/server

RUN cargo b --release

## release
FROM soerenmeier/chuchi-release

COPY --chown=release --from=0 /home/build/server/target/release/server .
COPY --chown=release --from=0 /home/build/ui/dist ui

CMD ["./server"]
