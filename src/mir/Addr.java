package mat.mir;
import mat.asm.Type;
import mat.asm.Operand;
import mat.asm.MemoryReference;
import mat.entity.Entity;

public class Addr extends Expr {
    Entity entity;

    public Addr(Type type, Entity entity) {
        super(type);
        this.entity = entity;
    }

    public boolean isAddr() { return true; }

    public Entity entity() { return entity; }

    public Operand address() {
        return entity.address();
    }

    public MemoryReference memref() {
        return entity.memref();
    }

    public Entity getEntityForce() {
        return entity;
    }

    public <S,E> E accept(MIRVisitor<S,E> visitor) {
        return visitor.visit(this);
    }

    protected void _dump(Dumper d) {
        d.printMember("entity", entity.name());
    }
}
