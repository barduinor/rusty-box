# MetadataCascadePolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the metadata cascade policy object | [optional]
**r#type** | Option<**String**> | `metadata_cascade_policy` | [optional]
**owner_enterprise** | Option<[**crate::models::MetadataCascadePolicyOwnerEnterprise**](MetadataCascadePolicy_owner_enterprise.md)> |  | [optional]
**parent** | Option<[**crate::models::MetadataCascadePolicyParent**](MetadataCascadePolicy_parent.md)> |  | [optional]
**scope** | Option<**String**> | The scope of the of the template that is cascaded down to the folder's children. | [optional]
**template_key** | Option<**String**> | The key of the template that is cascaded down to the folder's children.  In many cases the template key is automatically derived of its display name, for example `Contract Template` would become `contractTemplate`. In some cases the creator of the template will have provided its own template key.  Please [list the templates for an enterprise][list], or get all instances on a [file][file] or [folder][folder] to inspect a template's key.  [list]: e://get-metadata-templates-enterprise [file]: e://get-files-id-metadata [folder]: e://get-folders-id-metadata | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


