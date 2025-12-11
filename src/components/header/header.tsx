import { Link } from "@tanstack/react-router";
import { Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarSeparator, MenubarShortcut, MenubarTrigger } from "@/components/ui/menubar";

export function Header() {
    return (
        <header>
            <Menubar className="rounded-none p-0 border-0">
                <MenubarMenu>
                    <MenubarTrigger>File</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem asChild>
                            <Link to="/">
                                Home
                                <MenubarShortcut>⌘T</MenubarShortcut>
                            </Link>
                        </MenubarItem>
                        <MenubarSeparator />
                        <MenubarItem asChild>
                            <Link to="/settings">
                                Settings
                                <MenubarShortcut>⌘P</MenubarShortcut>
                            </Link>
                        </MenubarItem>
                        <MenubarSeparator />
                        <MenubarItem>
                            Exit
                            <MenubarShortcut>⌘W</MenubarShortcut>
                        </MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
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
