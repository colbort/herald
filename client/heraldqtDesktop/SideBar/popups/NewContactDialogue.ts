export function insertContact(
entryArea: TextArea, contactsModel: Users, networkHandle: NetworkHandle, conversationsModel: Conversations,
): void {
  const trimmedText = entryArea.text.trim();
    if (trimmedText.length === 0) {
        return;
    }
    const conversationId = contactsModel.add(trimmedText);
    networkHandle.sendAddRequest(trimmedText, conversationId);
    entryArea.clear();
    conversationsModel.hardRefresh();
}
