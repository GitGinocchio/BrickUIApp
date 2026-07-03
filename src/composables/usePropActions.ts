import type { Brick, Prop } from "~/interfaces/brick";




export const usePropActions = () => {
    const { saveBrick } = useBrickActions();
    const toast = useToast();

    const deleteProp = (brick: Brick, prop: Prop) => {
        brick.props = brick.props.filter(p => p !== prop);
        saveBrick(brick);
    }

    const duplicateProp = (brick: Brick, prop: Prop) => {
        const copyName = `${prop.prop_name}Copy`;
        if (brick.props.some(p => p.prop_name === copyName)) {
            toast.add({ title: 'Error', description: 'Copy already exists', color: 'error' });
            return;
        }

        const index = brick.props.findIndex(p => p.prop_name === prop.prop_name);
        brick.props.splice(index + 1, 0, { ...prop, prop_name: copyName });
        saveBrick(brick);
    }

    const changePropOrder = (brick: Brick, oldIndex: number, newIndex: number) => {
        if (oldIndex !== newIndex) {
            const newList = [...brick.props];
            const [movedItem] = newList.splice(oldIndex, 1);
            newList.splice(newIndex, 0, movedItem);
            brick.props = newList;
            saveBrick(brick);
        }
    }

    const openDeletePropModal = () => {

    }

    return {
        duplicateProp,
        deleteProp,
        changePropOrder,
        
        openDeletePropModal
    };
}