public class Held {
     String name;
     int stärke;
     int lebenspunkte;
     int angriffswert;
     Waffe waffe;
     Kampfregel regel;

    public  Held(String name, int stärke, int lebenspunkte, int angriffswert, Waffe waffe, Kampfregel regel) {

        this.name = name
        this.stärke = stärke
        this.lebenspunkte = lebenspunkte
        this.angriffswert = angriffswert
        this.waffe = waffe
        this.regel = regel
    }

     boolean addLebenspunkte(int faktor) {}
     void angreifen(Monster monster, Kampfregel regel) {}
    abstract  int getAngriffswert() {}
}
