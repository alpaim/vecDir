import { useNavigate } from "@tanstack/react-router";
import { MenubarCheckboxItem, MenubarContent, MenubarItem, MenubarMenu, MenubarSeparator, MenubarTrigger } from "@/components/ui/menubar";
import { useAppState } from "@/store/store";

export function Spaces() {
    const spaces = useAppState(state => state.spaces);
    const selectedSpace = useAppState(state => state.selectedSpace);
    const selectSpace = useAppState(state => state.selectSpace);

    const spacesList = Array.from(spaces.values());

    const navigate = useNavigate();

    return (
        <MenubarMenu>
            <MenubarTrigger>Spaces</MenubarTrigger>
            <MenubarContent>
                <MenubarItem onClick={() => {
                    navigate({ to: "/createSpace" });
                }}
                >
                    New
                </MenubarItem>
                <MenubarSeparator />
                {
                    spacesList.map(space => (
                        <MenubarCheckboxItem key={space.id} onClick={() => selectSpace(space.id)} checked={space.id === selectedSpace}>
                            {space.name}
                        </MenubarCheckboxItem>
                    ))
                }
            </MenubarContent>
        </MenubarMenu>
    );
}
