public class Held {
    protected String name;
    protected int stärke;
    protected int lebenspunkte;
    protected int angriffswert;
    protected Waffe waffe;

    public  Held(String name, int stärke, int lebenspunkte, int angriffswert, Waffe waffe) {

        this.name = name;
        this.stärke = stärke;
        this.lebenspunkte = lebenspunkte;
        this.angriffswert = angriffswert;
        this.waffe = waffe;
    }

    public boolean addLebenspunkte(int faktor) {}
    public void angreifen(Monster monster, Kampfregel regel) {}
    public abstract int getAngriffswert() {}
    private abstract void useweapon(Waffe w) {}
}
