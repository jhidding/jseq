# jseq
ALSA MIDI Sequencer in Rust

MIDI sequencers in Linux come in several flavours. There are those catering for techno, and those for writing classical music. Both usually suffer from horribly designed user interfaces. I want to have something else:

- Something that doesn't rely on Jack: setting up Jack is a nightmare; ALSA provides all I need.
- Something that works without a GUI: we can always stick a web-interface on top.
- Something scriptable

My goal is to have something intuitive to compose music with, real-time performance is not a priority.
