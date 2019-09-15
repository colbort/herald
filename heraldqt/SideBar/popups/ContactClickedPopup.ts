export function deleteContact(
  index: number,
  contactsModel: Contacts,
  messageModel: Messages,
  appRoot: GlobalState,
  heraldUtils: HeraldUtils
): void {
  const sameId = heraldUtils.compareByteArray(
    messageModel.conversationId,
    contactsModel.pairwiseConversationId(index)
  );

  if (sameId) {
    appRoot.gsConversationId = undefined;
  }

  contactsModel.setStatus(index, ContactStatus.Deleted);
}

export function renameContact(
  index: number,
  entryField: TextArea,
  renameContactDialogue: Popup,
  contactsModel: Contacts
): boolean {
  const trimmedText = entryField.text.trim();
  if (trimmedText === "") {
    return false;
  }

  const ret = contactsModel.setName(index, trimmedText);
  entryField.clear();
  renameContactDialogue.close();
  return ret;
}
