// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    DrivesListContentTypesApiClient,
    DrivesListContentTypesIdApiClient,
    ResourceIdentity::DrivesListContentTypes
);

impl DrivesListContentTypesApiClient {
    post!(
        doc: "Create new navigation property to contentTypes for drives",
        name: create_content_types,
        path: "/contentTypes",
        body: true
    );
    get!(
        doc: "List contentTypes in a list",
        name: list_content_types,
        path: "/contentTypes"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_content_types_count,
        path: "/contentTypes/$count"
    );
    post!(
        doc: "Invoke action addCopy",
        name: add_copy,
        path: "/contentTypes/addCopy",
        body: true
    );
    post!(
        doc: "Invoke action addCopyFromContentTypeHub",
        name: add_copy_from_content_type_hub,
        path: "/contentTypes/addCopyFromContentTypeHub",
        body: true
    );
    get!(
        doc: "Invoke function getCompatibleHubContentTypes",
        name: get_compatible_hub_content_types,
        path: "/contentTypes/getCompatibleHubContentTypes()"
    );
}

impl DrivesListContentTypesIdApiClient {
    delete!(
        doc: "Delete navigation property contentTypes for drives",
        name: delete_content_types,
        path: "/contentTypes/{{RID}}"
    );
    get!(
        doc: "Get contentTypes from drives",
        name: get_content_types,
        path: "/contentTypes/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property contentTypes in drives",
        name: update_content_types,
        path: "/contentTypes/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action associateWithHubSites",
        name: associate_with_hub_sites,
        path: "/contentTypes/{{RID}}/associateWithHubSites",
        body: true
    );
    get!(
        doc: "Get base from drives",
        name: get_base,
        path: "/contentTypes/{{RID}}/base"
    );
    get!(
        doc: "Get baseTypes from drives",
        name: list_base_types,
        path: "/contentTypes/{{RID}}/baseTypes"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_base_types_count,
        path: "/contentTypes/{{RID}}/baseTypes/$count"
    );
    get!(
        doc: "Get baseTypes from drives",
        name: get_base_types,
        path: "/contentTypes/{{RID}}/baseTypes/{{id}}",
        params: content_type_id_1
    );
    post!(
        doc: "Create new navigation property to columnLinks for drives",
        name: create_column_links,
        path: "/contentTypes/{{RID}}/columnLinks",
        body: true
    );
    get!(
        doc: "Get columnLinks from drives",
        name: list_column_links,
        path: "/contentTypes/{{RID}}/columnLinks"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_column_links_count,
        path: "/contentTypes/{{RID}}/columnLinks/$count"
    );
    delete!(
        doc: "Delete navigation property columnLinks for drives",
        name: delete_column_links,
        path: "/contentTypes/{{RID}}/columnLinks/{{id}}",
        params: column_link_id
    );
    get!(
        doc: "Get columnLinks from drives",
        name: get_column_links,
        path: "/contentTypes/{{RID}}/columnLinks/{{id}}",
        params: column_link_id
    );
    patch!(
        doc: "Update the navigation property columnLinks in drives",
        name: update_column_links,
        path: "/contentTypes/{{RID}}/columnLinks/{{id}}",
        body: true,
        params: column_link_id
    );
    get!(
        doc: "Get columnPositions from drives",
        name: list_column_positions,
        path: "/contentTypes/{{RID}}/columnPositions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_column_positions_count,
        path: "/contentTypes/{{RID}}/columnPositions/$count"
    );
    get!(
        doc: "Get columnPositions from drives",
        name: get_column_positions,
        path: "/contentTypes/{{RID}}/columnPositions/{{id}}",
        params: column_definition_id
    );
    post!(
        doc: "Create a columnDefinition in a content type",
        name: create_columns,
        path: "/contentTypes/{{RID}}/columns",
        body: true
    );
    get!(
        doc: "List columnDefinitions in a content type",
        name: list_columns,
        path: "/contentTypes/{{RID}}/columns"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_columns_count,
        path: "/contentTypes/{{RID}}/columns/$count"
    );
    delete!(
        doc: "Delete navigation property columns for drives",
        name: delete_columns,
        path: "/contentTypes/{{RID}}/columns/{{id}}",
        params: column_definition_id
    );
    get!(
        doc: "Get columns from drives",
        name: get_columns,
        path: "/contentTypes/{{RID}}/columns/{{id}}",
        params: column_definition_id
    );
    patch!(
        doc: "Update the navigation property columns in drives",
        name: update_columns,
        path: "/contentTypes/{{RID}}/columns/{{id}}",
        body: true,
        params: column_definition_id
    );
    get!(
        doc: "Get sourceColumn from drives",
        name: get_source_column,
        path: "/contentTypes/{{RID}}/columns/{{id}}/sourceColumn",
        params: column_definition_id
    );
    post!(
        doc: "Invoke action copyToDefaultContentLocation",
        name: copy_to_default_content_location,
        path: "/contentTypes/{{RID}}/copyToDefaultContentLocation",
        body: true
    );
    get!(
        doc: "Invoke function isPublished",
        name: is_published,
        path: "/contentTypes/{{RID}}/isPublished()"
    );
    post!(
        doc: "Invoke action publish",
        name: publish,
        path: "/contentTypes/{{RID}}/publish"
    );
    post!(
        doc: "Invoke action unpublish",
        name: unpublish,
        path: "/contentTypes/{{RID}}/unpublish"
    );
}
