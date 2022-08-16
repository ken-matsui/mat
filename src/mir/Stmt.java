package mat.mir;

import mat.ast.Location;

abstract public class Stmt implements Dumpable {
    protected Location location;

    public Stmt(Location loc) {
        this.location = loc;
    }

    abstract public <S,E> S accept(MIRVisitor<S,E> visitor);

    public Location location() {
        return location;
    }

    public void dump(Dumper d) {
        d.printClass(this, location);
        _dump(d);
    }

    abstract protected void _dump(Dumper d);
}
