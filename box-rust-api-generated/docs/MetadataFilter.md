# MetadataFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scope** | Option<**String**> | Specifies the scope of the template to filter search results by.  This will be `enterprise_{enterprise_id}` for templates defined for use in this enterprise, and `global` for general templates that are available to all enterprises using Box. | [optional]
**template_key** | Option<**String**> | The key of the template to filter search results by.  In many cases the template key is automatically derived of its display name, for example `Contract Template` would become `contractTemplate`. In some cases the creator of the template will have provided its own template key.  Please [list the templates for an enterprise][list], or get all instances on a [file][file] or [folder][folder] to inspect a template's key.  [list]: e://get-metadata-templates-enterprise [file]: e://get-files-id-metadata [folder]: e://get-folders-id-metadata | [optional]
**filters** | Option<[**crate::models::MetadataFilterFilters**](MetadataFilter_filters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


