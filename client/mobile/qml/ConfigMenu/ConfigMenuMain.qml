import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "./Controls"
import "../Common" as Common

Page {
    id: configPage
    header: ConfigHeader {}

    ColumnLayout {
        id: configContent
        width: parent.width
        ConfigSection {
            title: qsTr("Account")
            content: Column {

                Row {
                    Common.HeaderText {
                        font.pixelSize: 18
                        text: qsTr("Username:") + " "
                        leftPadding: CmnCfg.margin
                    }
                    Common.HeaderText {
                        font.pixelSize: 18
                        text: Herald.config.name
                        color: CmnCfg.palette.darkGrey
                    }
                }

                Common.HeaderText {
                    font.pixelSize: 12
                    text: qsTr("The primary identifying contact information of your account,\
any of your contacts must know this identifier exactly. \
It is not searchable, nor can it be changed.")
                    color: CmnCfg.palette.darkGrey
                    wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                    leftPadding: CmnCfg.margin
                    width: parent.width
                }
            }
        }

        ConfigSection {
            title: qsTr("Appearance")
        }

        ConfigSection {
            title: qsTr("Notifications")
        }

        ConfigSection {
            title: qsTr("Advanced")
        }

        ConfigSection {
            title: qsTr("About")
        }
    }
}
