fn parse_meta(document: &Document) -> SchemaMeta {
    return SchemaMeta {
        version: find_version(&document)
    }
}