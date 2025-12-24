import { useNavigate } from "@tanstack/react-router";
import { Spaces } from "@/components/header/spaces";
import { Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarSeparator, MenubarShortcut, MenubarTrigger } from "@/components/ui/menubar";

export function Header() {
    const navigate = useNavigate();

    return (
        <header>
            <Menubar className="rounded-none p-0 border-0">
                <MenubarMenu>
                    <MenubarTrigger>File</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem
                            asChild
                            onClick={() => {
                                navigate({ to: "/" });
                            }}
                        >
                            Home
                            <MenubarShortcut>⌘T</MenubarShortcut>

                        </MenubarItem>
                        <MenubarSeparator />
                        <MenubarItem
                            asChild
                            onClick={() => {
                                navigate({ to: "/settings" });
                            }}
                        >
                            Settings
                            <MenubarShortcut>⌘P</MenubarShortcut>

                        </MenubarItem>
                        <MenubarSeparator />
                        <MenubarItem>
                            Exit
                            <MenubarShortcut>⌘W</MenubarShortcut>
                        </MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <Spaces />
                <MenubarMenu>
                    <MenubarTrigger>Help</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem>
                            Open GitHub repository
                        </MenubarItem>
                        <MenubarItem>
                            Check for updates
                        </MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>
        </header>
    );
}
