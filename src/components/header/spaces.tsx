import { Link } from "@tanstack/react-router";
import { MenubarCheckboxItem, MenubarContent, MenubarItem, MenubarMenu, MenubarSeparator, MenubarTrigger } from "@/components/ui/menubar";
import { useAppState } from "@/store/store";

export function Spaces() {
    const spaces = useAppState(state => state.spaces);
    const selectedSpace = useAppState(state => state.selectedSpace);
    const selectSpace = useAppState(state => state.selectSpace);

    const spacesList = Array.from(spaces.values());

    Object.values(spaces).map((space) => {
        console.log(space);
        return space;
    });

    return (
        <MenubarMenu>
            <MenubarTrigger>Spaces</MenubarTrigger>
            <MenubarContent>
                <MenubarItem>
                    <Link to="/createSpace" className="w-full">
                        New
                    </Link>
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
