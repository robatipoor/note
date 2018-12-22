FROM ubuntu:latest
ADD target/release/note-rs /
CMD ["/note-rs"]
