# Experimental source-provenance and non-destructive plaintext zoning

## Implemented, separate from OCEL

`mailweft-core::inspect_body_zones(raw)` accepts one bounded raw RFC 5322/MIME source and produces a **content-free evidence sidecar**, without changing the Stage 1 observed-event OCEL or inferring process events. CLI: `mailweft inspect source.eml > source-zones.json`. The sidecar records the source SHA-256, original MIME part identifier, raw-body offset range/hash, decoded text length/hash, decoding-problem flag and an ordered list of zones. For each zone: conservative type, decoded-byte offsets, zone SHA-256 and **exact** raw byte offsets only when the decoded content equals the raw source slice and decoding reported no problems.

If charset/transfer decoding changes the bytes, the zone's raw offsets are `null` and its quality is `derived`. The source-part byte range remains known. This distinction is required: UTF-8 offsets in the decoded text are not automatically offsets in encoded EML bytes.

The deterministic, intentionally low-recall baseline identifies a narrow set of salutations (`Hi/Hello/Dear ...,` on the first content line), quote-prefixed lines (`>`), common sign-offs, post-signoff signature lines, selected disclaimer prefixes, and forwarded markers. Blank lines and unclassified material are labelled `unknown`. **Every decoded plaintext line is retained as a zone**, with no destructive stripping or promotion to a new business event. A designation is a candidate structural label, not a claim about who authored the content.

Only actual parser `PartKind::Text` bodies are zoned. An HTML-only message or text-body fallback remains skipped; no HTML renderer, remote image retrieval, link fetching, OCR, LLM, graph search or attachment execution is invoked. This is intentionally a **primitive structural baseline**, not Quagga/Zebra parity or comprehensive inline/forwarded nested zoning.

## Privacy and evidence properties

No addresses, names, body text or filenames are emitted in the sidecar. Hashes, source offsets and zone metadata are nevertheless **pseudonymous, not anonymous**: a low-entropy source or line can be guessed and hashed. Keep the sidecar scoped to the source owner's authorisation and preserve original evidence separately. This API is not itself a source archive.

Exact offsets can be verified against the original EML for plain, undecoded UTF-8 source. Derived offsets cannot be interpreted as raw bytes; future character/transfer mapping must be independently verified before enabling precise cross-encoding citations.

## Correctness repair included in this change

Prior 0.3 code could construct `mime_attachment` OCEL objects with named attributes without listing `mime_attachment` in `objectTypes`. The 0.4 projection now declares `mime_attachment` with `declared_media_type`, `filename_sha256` and `decoded_byte_length` typed attribute definitions. A regression test checks that every constructed OCEL object and object attribute is declared. Successful Rust4PM import alone did not reveal this problem; schema-completeness regression tests are therefore required in addition to engine-compatibility tests.

## Outstanding limitations

- No source/rights envelope, externally attested acquisition timestamp or archival completeness manifest.
- No true raw-byte-to-decoded-character mapping for base64, quoted-printable, malformed UTF-8 or HTML alternatives.
- No nested zone graph, multiple possible authors, conversation context, in-text person assignment, signature role resolution, local NLP, GraphRAG or human adjudication.
- Simple line rules can misclassify a sentence that superficially resembles a signoff, greeting, quoted passage or forwarded marker; uncertain passages should not be promoted to verified events.
- No large-corpus zoning benchmark, multilingual evaluation, fuzzed MIME corpus or privacy-hardening through scoped secret-keyed identifiers.
- Source-dependent `sha256` hashes and exact raw offsets are intended for authorised analysis, not public disclosure of sensitive correspondence.

Next: build a verified source-envelope and incremental textual evidence representation, evaluate controlled zoning errors, then thread candidates without conflating header references with observed correspondence.
