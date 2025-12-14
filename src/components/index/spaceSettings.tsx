import { RootsList } from "@/components/index/rootsList";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";

export function SpaceSettings() {
    return (
        <div className="flex justify-around w-full">
            <Card className="w-full max-w-3xs">
                <CardHeader>
                    <CardTitle>
                        Directories
                    </CardTitle>
                </CardHeader>
                <CardContent>
                    <RootsList roots={[]} />
                </CardContent>
                <CardFooter className="flex-col gap-2">
                    <Button
                        variant="outline"
                        className="w-full"
                        onClick={() => {}}
                    >
                        Add Directory
                    </Button>
                </CardFooter>
            </Card>
            <Card className="w-full max-w-3xs">
                <CardHeader>
                    <CardTitle>
                        AI Settings
                    </CardTitle>
                </CardHeader>
                <CardContent>
                    asdas
                </CardContent>
            </Card>
        </div>
    );
}
