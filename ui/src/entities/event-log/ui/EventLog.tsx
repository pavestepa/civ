import { eventLogSelectors, useEventLogStore } from "../model/store";

export function EventLog() {
  const entries = useEventLogStore(eventLogSelectors.entries);

  return (
    <aside className="hud-log">
      {entries.map((entry, index) => (
        <div key={`${entry}-${index}`} className="log-entry">
          {entry}
        </div>
      ))}
    </aside>
  );
}
