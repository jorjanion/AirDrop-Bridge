import plistlib
import uuid
import re

def u():
    return str(uuid.uuid4()).upper()

def make_token_string(template, attachments_map):
    pieces = re.split(r'(\{[a-zA-Z0-9_]+\})', template)
    res_str = ""
    attachments_by_range = {}
    for p in pieces:
        if p.startswith('{') and p.endswith('}'):
            var_name = p[1:-1]
            if var_name in attachments_map:
                idx = len(res_str)
                res_str += "\ufffc"
                attachments_by_range[f"{{{idx}, 1}}"] = attachments_map[var_name]
            else:
                res_str += p
        else:
            res_str += p
    return {
        "WFSerializationType": "WFTextTokenString",
        "Value": {
            "string": res_str,
            "attachmentsByRange": attachments_by_range
        }
    }

def action_output(uuid_str, output_name, dict_key=None):
    att = {
        "Type": "ActionOutput",
        "OutputUUID": uuid_str,
        "OutputName": output_name
    }
    if dict_key:
        att["Aggrandizements"] = [
            {
                "Type": "WFDictionaryValueVariableAggrandizement",
                "DictionaryKey": dict_key
            }
        ]
    return att

def build_shortcut():
    actions = []

    # 1. Comment action
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.comment",
        "WFWorkflowActionParameters": {
            "WFCommentActionText": "AirDrop Bridge\nZero-config local file transfer and clipboard sync for Windows and Linux."
        }
    })

    # 2. Default Config Dictionary
    default_config_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.dictionary",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "DefaultConfig",
            "UUID": default_config_uuid,
            "WFItems": {
                "Value": {
                    "WFDictionaryFieldValueItems": [
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "Host"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": {"Value": {"string": "AIRDROP_BRIDGE_HOST"}, "WFSerializationType": "WFTextTokenString"}
                        },
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "IP"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": {"Value": {"string": "AIRDROP_BRIDGE_IP"}, "WFSerializationType": "WFTextTokenString"}
                        },
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "Port"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": {"Value": {"string": "53317"}, "WFSerializationType": "WFTextTokenString"}
                        },
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "Token"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": {"Value": {"string": "AIRDROP_BRIDGE_TOKEN"}, "WFSerializationType": "WFTextTokenString"}
                        },
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "DeviceName"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": {"Value": {"string": "AirDrop Bridge PC"}, "WFSerializationType": "WFTextTokenString"}
                        }
                    ]
                },
                "WFSerializationType": "WFDictionaryFieldValue"
            }
        }
    })

    # 3. Check if input is a setup payload from Step 2
    detect_input_dict_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.detect.dictionary",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "InputDict",
            "UUID": detect_input_dict_uuid,
            "WFInput": {
                "Value": {"Type": "ExtensionInput"},
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })

    get_input_token_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.getvalueforkey",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "InputToken",
            "UUID": get_input_token_uuid,
            "WFDictionaryKey": "token",
            "WFInput": {
                "Value": action_output(detect_input_dict_uuid, "InputDict"),
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })

    gid_setup_check = u()
    # If token exists in input -> Setup mode
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.conditional",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_setup_check,
            "WFCondition": 100, # Has any value
            "WFControlFlowMode": 0,
            "WFInput": {
                "Type": "Variable",
                "Variable": {
                    "Value": action_output(get_input_token_uuid, "InputToken"),
                    "WFSerializationType": "WFTextTokenAttachment"
                }
            }
        }
    })

    # Normalize keys: Host = host, Port = port, Token = token, DeviceName = device_name
    get_input_host_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.getvalueforkey",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "SetupHost",
            "UUID": get_input_host_uuid,
            "WFDictionaryKey": "host",
            "WFInput": {
                "Value": action_output(detect_input_dict_uuid, "InputDict"),
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })

    get_input_port_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.getvalueforkey",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "SetupPort",
            "UUID": get_input_port_uuid,
            "WFDictionaryKey": "port",
            "WFInput": {
                "Value": action_output(detect_input_dict_uuid, "InputDict"),
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })

    get_input_device_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.getvalueforkey",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "SetupDevice",
            "UUID": get_input_device_uuid,
            "WFDictionaryKey": "device_name",
            "WFInput": {
                "Value": action_output(detect_input_dict_uuid, "InputDict"),
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })

    save_dict_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.dictionary",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "SavedConfigDict",
            "UUID": save_dict_uuid,
            "WFItems": {
                "Value": {
                    "WFDictionaryFieldValueItems": [
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "Host"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": make_token_string("{SetupHost}", {"SetupHost": action_output(get_input_host_uuid, "SetupHost")})
                        },
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "Port"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": make_token_string("{SetupPort}", {"SetupPort": action_output(get_input_port_uuid, "SetupPort")})
                        },
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "Token"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": make_token_string("{InputToken}", {"InputToken": action_output(get_input_token_uuid, "InputToken")})
                        },
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "DeviceName"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": make_token_string("{SetupDevice}", {"SetupDevice": action_output(get_input_device_uuid, "SetupDevice")})
                        }
                    ]
                },
                "WFSerializationType": "WFDictionaryFieldValue"
            }
        }
    })

    # Save to iCloud Drive/Shortcuts/AirDropBridge.json
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.documentpicker.save",
        "WFWorkflowActionParameters": {
            "WFAskWhereToSave": False,
            "WFFileDestinationPath": "AirDropBridge.json",
            "WFFileStorageService": "iCloud Drive",
            "WFInput": {
                "Value": action_output(save_dict_uuid, "SavedConfigDict"),
                "WFSerializationType": "WFTextTokenAttachment"
            },
            "WFSaveFileOverwrite": True
        }
    })

    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.alert",
        "WFWorkflowActionParameters": {
            "WFAlertActionCancelButtonShown": False,
            "WFAlertActionMessage": make_token_string("AirDrop Bridge is now connected to {SetupDevice}!\nHost: {SetupHost}:{SetupPort}", {
                "SetupDevice": action_output(get_input_device_uuid, "SetupDevice"),
                "SetupHost": action_output(get_input_host_uuid, "SetupHost"),
                "SetupPort": action_output(get_input_port_uuid, "SetupPort")
            }),
            "WFAlertActionTitle": "AirDrop Bridge Configured"
        }
    })

    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.exit",
        "WFWorkflowActionParameters": {}
    })

    # End setup check
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.conditional",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_setup_check,
            "WFControlFlowMode": 2
        }
    })

    # 4. Open saved config if present
    open_config_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.documentpicker.open",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "SavedConfigFile",
            "UUID": open_config_uuid,
            "WFDocumentPickerShowFilePicker": False,
            "WFFileStorageService": "iCloud Drive",
            "WFGetFilePath": "AirDropBridge.json"
        }
    })

    gid_config_pick = u()
    if_config_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.conditional",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_config_pick,
            "WFCondition": 100, # SavedConfigFile has value
            "WFControlFlowMode": 0,
            "WFInput": {
                "Type": "Variable",
                "Variable": {
                    "Value": action_output(open_config_uuid, "SavedConfigFile"),
                    "WFSerializationType": "WFTextTokenAttachment"
                }
            }
        }
    })

    parse_saved_dict_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.detect.dictionary",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "ActiveConfig",
            "UUID": parse_saved_dict_uuid,
            "WFInput": {
                "Value": action_output(open_config_uuid, "SavedConfigFile"),
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })

    # Otherwise: use DefaultConfig
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.conditional",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_config_pick,
            "WFControlFlowMode": 1
        }
    })

    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.detect.dictionary",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "ActiveConfig",
            "UUID": u(),
            "WFInput": {
                "Value": action_output(default_config_uuid, "DefaultConfig"),
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })

    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.conditional",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_config_pick,
            "UUID": if_config_uuid,
            "WFControlFlowMode": 2
        }
    })

    active_config_att = action_output(if_config_uuid, "ActiveConfig")

    # Extract Host, Port, Token, DeviceName
    host_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.getvalueforkey",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "Host",
            "UUID": host_uuid,
            "WFDictionaryKey": "Host",
            "WFInput": {
                "Value": active_config_att,
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })
    host_att = action_output(host_uuid, "Host")

    port_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.getvalueforkey",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "Port",
            "UUID": port_uuid,
            "WFDictionaryKey": "Port",
            "WFInput": {
                "Value": active_config_att,
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })
    port_att = action_output(port_uuid, "Port")

    token_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.getvalueforkey",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "Token",
            "UUID": token_uuid,
            "WFDictionaryKey": "Token",
            "WFInput": {
                "Value": active_config_att,
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })
    token_att = action_output(token_uuid, "Token")

    device_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.getvalueforkey",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "DeviceName",
            "UUID": device_uuid,
            "WFDictionaryKey": "DeviceName",
            "WFInput": {
                "Value": active_config_att,
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })
    device_att = action_output(device_uuid, "DeviceName")

    # 5. Check if we have Shortcut Input (Files / Photos / Text)
    gid_main = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.conditional",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_main,
            "WFCondition": 100, # Shortcut input has any value
            "WFControlFlowMode": 0,
            "WFInput": {
                "Type": "Variable",
                "Variable": {
                    "Value": {"Type": "ExtensionInput"},
                    "WFSerializationType": "WFTextTokenAttachment"
                }
            }
        }
    })

    # --- BRANCH A: SHARE SHEET ---
    gid_repeat = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.repeat.each",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_repeat,
            "WFControlFlowMode": 0,
            "WFInput": {
                "Value": {"Type": "ExtensionInput"},
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })

    # Send file upload POST /api/v1/upload
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.downloadurl",
        "WFWorkflowActionParameters": {
            "ShowHeaders": True,
            "UUID": u(),
            "WFHTTPBodyType": "Form",
            "WFFormValues": {
                "Value": {
                    "WFDictionaryFieldValueItems": [
                        {
                            "WFItemType": 5, # File/Attachment
                            "WFKey": {"Value": {"string": "file"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": {
                                "Value": {
                                    "Value": {
                                        "Aggrandizements": [
                                            {
                                                "CoercionItemClass": "WFGenericFileContentItem",
                                                "Type": "WFCoercionVariableAggrandizement"
                                            }
                                        ],
                                        "Type": "Variable",
                                        "VariableName": "Repeat Item"
                                    },
                                    "WFSerializationType": "WFTextTokenAttachment"
                                },
                                "WFSerializationType": "WFTokenAttachmentParameterState"
                            }
                        }
                    ]
                },
                "WFSerializationType": "WFDictionaryFieldValue"
            },
            "WFHTTPHeaders": {
                "Value": {
                    "WFDictionaryFieldValueItems": [
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "Authorization"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": make_token_string("Bearer {Token}", {"Token": token_att})
                        }
                    ]
                },
                "WFSerializationType": "WFDictionaryFieldValue"
            },
            "WFHTTPMethod": "POST",
            "WFURL": make_token_string("http://{Host}:{Port}/api/v1/upload", {"Host": host_att, "Port": port_att})
        }
    })

    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.repeat.each",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_repeat,
            "WFControlFlowMode": 2
        }
    })

    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.notification",
        "WFWorkflowActionParameters": {
            "WFNotificationActionBody": make_token_string("Files sent successfully to {DeviceName}!", {"DeviceName": device_att}),
            "WFNotificationActionTitle": "AirDrop Bridge"
        }
    })

    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.exit",
        "WFWorkflowActionParameters": {}
    })

    # --- BRANCH B: RUN DIRECTLY (CLIPBOARD & STATUS) ---
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.conditional",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_main,
            "WFControlFlowMode": 1
        }
    })

    gid_menu = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.choosefrommenu",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_menu,
            "WFControlFlowMode": 0,
            "WFMenuItems": [
                "📤 Send Clipboard to PC",
                "📥 Get Clipboard from PC",
                "🔍 Check Connection"
            ],
            "WFMenuTitle": "AirDrop Bridge"
        }
    })

    # Menu item 1: Send Clipboard
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.choosefrommenu",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_menu,
            "WFControlFlowMode": 1,
            "WFMenuItemTitle": "📤 Send Clipboard to PC"
        }
    })

    clip_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.getclipboard",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "ClipboardContent",
            "UUID": clip_uuid
        }
    })
    clip_att = action_output(clip_uuid, "ClipboardContent")

    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.downloadurl",
        "WFWorkflowActionParameters": {
            "ShowHeaders": True,
            "UUID": u(),
            "WFHTTPBodyType": "JSON",
            "WFJSONValues": {
                "Value": {
                    "WFDictionaryFieldValueItems": [
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "type"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": {"Value": {"string": "text"}, "WFSerializationType": "WFTextTokenString"}
                        },
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "content"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": make_token_string("{Content}", {"Content": clip_att})
                        }
                    ]
                },
                "WFSerializationType": "WFDictionaryFieldValue"
            },
            "WFHTTPHeaders": {
                "Value": {
                    "WFDictionaryFieldValueItems": [
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "Authorization"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": make_token_string("Bearer {Token}", {"Token": token_att})
                        },
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "Content-Type"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": {"Value": {"string": "application/json"}, "WFSerializationType": "WFTextTokenString"}
                        }
                    ]
                },
                "WFSerializationType": "WFDictionaryFieldValue"
            },
            "WFHTTPMethod": "POST",
            "WFURL": make_token_string("http://{Host}:{Port}/api/v1/clipboard", {"Host": host_att, "Port": port_att})
        }
    })

    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.notification",
        "WFWorkflowActionParameters": {
            "WFNotificationActionBody": make_token_string("Clipboard sent to {DeviceName}!", {"DeviceName": device_att}),
            "WFNotificationActionTitle": "AirDrop Bridge"
        }
    })

    # Menu item 2: Get Clipboard
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.choosefrommenu",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_menu,
            "WFControlFlowMode": 1,
            "WFMenuItemTitle": "📥 Get Clipboard from PC"
        }
    })

    get_clip_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.downloadurl",
        "WFWorkflowActionParameters": {
            "ShowHeaders": True,
            "UUID": get_clip_uuid,
            "CustomOutputName": "ClipboardResponse",
            "WFHTTPHeaders": {
                "Value": {
                    "WFDictionaryFieldValueItems": [
                        {
                            "WFItemType": 0,
                            "WFKey": {"Value": {"string": "Authorization"}, "WFSerializationType": "WFTextTokenString"},
                            "WFValue": make_token_string("Bearer {Token}", {"Token": token_att})
                        }
                    ]
                },
                "WFSerializationType": "WFDictionaryFieldValue"
            },
            "WFHTTPMethod": "GET",
            "WFURL": make_token_string("http://{Host}:{Port}/api/v1/clipboard", {"Host": host_att, "Port": port_att})
        }
    })

    get_clip_att = action_output(get_clip_uuid, "ClipboardResponse")
    clip_text_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.getvalueforkey",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "ClipboardText",
            "UUID": clip_text_uuid,
            "WFDictionaryKey": "content",
            "WFInput": {
                "Value": get_clip_att,
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })

    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.setclipboard",
        "WFWorkflowActionParameters": {
            "WFInput": {
                "Value": action_output(clip_text_uuid, "ClipboardText"),
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })

    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.notification",
        "WFWorkflowActionParameters": {
            "WFNotificationActionBody": make_token_string("Copied clipboard from {DeviceName}!", {"DeviceName": device_att}),
            "WFNotificationActionTitle": "AirDrop Bridge"
        }
    })

    # Menu item 3: Check Connection
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.choosefrommenu",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_menu,
            "WFControlFlowMode": 1,
            "WFMenuItemTitle": "🔍 Check Connection"
        }
    })

    ping_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.downloadurl",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "PingResponse",
            "UUID": ping_uuid,
            "WFHTTPMethod": "GET",
            "WFURL": make_token_string("http://{Host}:{Port}/api/v1/ping", {"Host": host_att, "Port": port_att})
        }
    })

    ping_status_uuid = u()
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.getvalueforkey",
        "WFWorkflowActionParameters": {
            "CustomOutputName": "PingStatus",
            "UUID": ping_status_uuid,
            "WFDictionaryKey": "status",
            "WFInput": {
                "Value": action_output(ping_uuid, "PingResponse"),
                "WFSerializationType": "WFTextTokenAttachment"
            }
        }
    })

    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.alert",
        "WFWorkflowActionParameters": {
            "WFAlertActionCancelButtonShown": False,
            "WFAlertActionMessage": make_token_string("Status: {Status}\nConnected to: {DeviceName}", {
                "Status": action_output(ping_status_uuid, "PingStatus"),
                "DeviceName": device_att
            }),
            "WFAlertActionTitle": "AirDrop Bridge Status"
        }
    })

    # End Menu
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.choosefrommenu",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_menu,
            "WFControlFlowMode": 2
        }
    })

    # End Main Conditional
    actions.append({
        "WFWorkflowActionIdentifier": "is.workflow.actions.conditional",
        "WFWorkflowActionParameters": {
            "GroupingIdentifier": gid_main,
            "WFControlFlowMode": 2
        }
    })

    shortcut_plist = {
        "WFWorkflowName": "AirDrop Bridge",
        "WFQuickActionSurfaces": [],
        "WFWorkflowActions": actions,
        "WFWorkflowClientVersion": "2610",
        "WFWorkflowHasOutputFallback": False,
        "WFWorkflowHasShortcutInputVariables": True,
        "WFWorkflowIcon": {
            "WFWorkflowIconGlyphNumber": 61501,
            "WFWorkflowIconStartColor": 4282601983
        },
        "WFWorkflowImportQuestions": [],
        "WFWorkflowInputContentItemClasses": [
            "WFGenericFileContentItem",
            "WFImageContentItem",
            "WFAVAssetContentItem",
            "WFPDFContentItem",
            "WFStringContentItem",
            "WFURLContentItem"
        ],
        "WFWorkflowMinimumClientVersion": 900,
        "WFWorkflowMinimumClientVersionString": "900",
        "WFWorkflowOutputContentItemClasses": [],
        "WFWorkflowTypes": ["ActionExtension", "QuickAction"]
    }

    return shortcut_plist

if __name__ == "__main__":
    plist_data = build_shortcut()
    out_path = "resources/shortcut/AirDrop_Bridge.shortcut"
    with open(out_path, "wb") as f:
        plistlib.dump(plist_data, f, fmt=plistlib.FMT_BINARY)
    print(f"Successfully generated {out_path} ({len(plist_data['WFWorkflowActions'])} actions)")
