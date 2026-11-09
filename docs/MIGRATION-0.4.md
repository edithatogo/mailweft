# 0.4 experimental contract migration

Packages `mailweft-core` and `mailweft-cli` advance from 0.3.0 to **0.4.0**. The Rust `Conversion.schema_version` advances from `1.2.0` to **`1.3.0`** because emitted OCEL type declarations change, even though the OCEL 2.0 root format remains unchanged.

A previous format defect omitted `mime_attachment` from the OCEL `objectTypes` catalogue despite emitting attachment objects with timestamped attributes. This change restores the declaration and typed attribute names. Consumers comparing JSON byte-for-byte will see the new type declaration; consumers validating full OCEL type completeness should now accept those attachment objects. Upgrade schema/type allowlists accordingly.

A new **independent, optional** API `inspect_body_zones(raw)` returns a versioned `BodyEvidence` sidecar with hashes, source-part offsets, decoded-zone offsets, offset provenance status and preliminary structural zone labels. The corresponding CLI mode is `mailweft inspect source.eml`. It is **not included inside the OCEL output**, does not modify observation identities or timestamps, and should be kept under the same evidence access controls as the raw source.

Use the sidecar only as a preliminary segmentation baseline. In particular, `derived` means that raw byte spans cannot be asserted from decoded offsets; labels are not verified actor identities or business events. No source/rights integration or automatic FOI-O ontology update is introduced. The default Rust converter remains independent of models, Python, GraphRAG and Enron.

Compatibility checks must include positive/negative synthetic MIME fixtures, the attachment declaration regression, exact/derived byte-offset tests, no-sensitive-text output tests, and Rust4PM import of canonical source observations.
