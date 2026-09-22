import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { CopyButton } from "@/components/CopyButton";

export type InstallMethod = { id: string; label: string; note: string; command: string };

/** Renders on the server with the first tab open; hydrates only for switching + copy. */
export function InstallTabs({ methods }: { methods: InstallMethod[] }) {
  return (
    <Tabs defaultValue={methods[0].id}>
      <TabsList aria-label="Install method">
        {methods.map((m) => (
          <TabsTrigger key={m.id} value={m.id}>
            {m.label}
          </TabsTrigger>
        ))}
      </TabsList>
      {methods.map((m) => (
        <TabsContent key={m.id} value={m.id}>
          <div className="relative group">
            <pre className="bg-card border border-border rounded-md p-4 pr-12 overflow-x-auto text-[0.85rem] leading-6">
              <code>
                {m.command.split("\n").map((line, i) => (
                  <span key={i} className="block">
                    {line.startsWith("#") ? (
                      <span className="text-dim">{line}</span>
                    ) : (
                      <>
                        <span className="text-dim select-none">$ </span>
                        {line}
                      </>
                    )}
                  </span>
                ))}
              </code>
            </pre>
            <CopyButton
              text={m.command
                .split("\n")
                .filter((l) => !l.startsWith("#"))
                .join("\n")}
              className="absolute top-2 right-2 text-muted-foreground"
            />
          </div>
          <p className="text-xs text-muted-foreground mt-2">{m.note}</p>
        </TabsContent>
      ))}
    </Tabs>
  );
}
