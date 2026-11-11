export function isListenerComplete(item: IListener): boolean {
 return !!item.title.trim() && item.monitors.length > 0 && item.rules.length > 0 && item.actions.length > 0
  && item.rules.every(rule => rule.search === 'FileSize' || rule.data.text.some(text => text.trim().length > 0))
  && item.actions.every(action => ['DELETE', 'UNLINK'].includes(action.action) || !!action.path?.trim());
}
