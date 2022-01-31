// Source: https://raw.githubusercontent.com/you21979/typescript-vector/master/vector3.ts
export interface Vec3 {
    x: number;
    y: number;
    z: number;
}

export class Vector3 implements Vec3 {
    public static create(v1: number | Vec3): Vector3 {
        if (typeof v1 === 'number') {
            return new Vector3(v1, v1, v1);
        }
        return new Vector3(v1.x, v1.y, v1.z);
    }

    public static clone(v1: Vec3): Vector3 {
        return Vector3.create(v1);
    }

    public static add(v1: Vec3, v2: number | Vec3): Vector3 {
        if (typeof v2 === 'number') {
            return new Vector3(v1.x + v2, v1.y + v2, v1.z + v2);
        }
        return new Vector3(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z);
    }

    public static subtract(v1: Vec3, v2: Vec3): Vector3 {
        return new Vector3(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z);
    }

    public static multiply(v1: Vec3, v2: Vec3 | number): Vector3 {
        if (typeof v2 === 'number') {
            return new Vector3(v1.x * v2, v1.y * v2, v1.z * v2);
        }
        return new Vector3(v1.x * v2.x, v1.y * v2.y, v1.z * v2.z);
    }

    public static divide(v1: Vec3, v2: Vec3 | number): Vector3 {
        if (typeof v2 === 'number') {
            return new Vector3(v1.x / v2, v1.y / v2, v1.z / v2);
        }
        return new Vector3(v1.x / v2.x, v1.y / v2.y, v1.z / v2.z);
    }

    public static dotProduct(v1: Vec3, v2: Vec3): number {
        return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    }

    public static crossProduct(v1: Vec3, v2: Vec3): Vector3 {
        const x = v1.y * v2.z - v1.z * v2.y;
        const y = v1.z * v2.x - v1.z * v2.z;
        const z = v1.x * v2.y - v1.z * v2.x;
        return new Vector3(x, y, z);
    }

    public static normalize(v: Vector3): Vector3 {
        return Vector3.divide(v, v.Length);
    }

    constructor(public x: number, public y: number, public z: number) {}

    public clone(): Vector3 {
        return new Vector3(this.x, this.y, this.z);
    }

    /**
     * The product of the Euclidean magnitudes of this and another Vector3.
     *
     * @param v Vector3 to find Euclidean magnitude between.
     * @returns Euclidean magnitude with another vector.
     */
    public distanceSquared(v: Vec3): number {
        const w: Vector3 = this.subtract(v);
        return Vector3.dotProduct(w, w);
    }

    /**
     * The distance between two Vectors.
     *
     * @param v Vector3 to find distance between.
     * @returns Distance between this and another vector.
     */
    public distance(v: Vec3): number {
        return Math.sqrt(this.distanceSquared(v));
    }

    public get normalize(): Vector3 {
        return Vector3.normalize(this);
    }

    public crossProduct(v: Vec3): Vector3 {
        return Vector3.crossProduct(this, v);
    }

    public dotProduct(v: Vec3): number {
        return Vector3.dotProduct(this, v);
    }

    public add(v: number | Vec3): Vec3 {
        return Vector3.add(this, v);
    }

    public subtract(v: Vec3): Vector3 {
        return Vector3.subtract(this, v);
    }

    public multiply(v: number | Vec3): Vector3 {
        return Vector3.multiply(this, v);
    }

    public divide(v: number | Vec3): Vec3 {
        return Vector3.divide(this, v);
    }

    public replace(v: Vec3): void {
        this.x = v.x;
        this.y = v.y;
        this.z = v.z;
    }

    public get Length(): number {
        return Math.sqrt(this.x * this.x + this.y * this.y + this.z * this.z);
    }
}


export function useVector3(x: number, y: number, z: number) {
return new Vector3(x, y, z)
}
export function useVector2(x: number, y: number) {
    return new Vector2(x, y)
}

export function useVectorialDistance<T extends Vector2 | Vector3>(v1: T, v2: T) {
    if (v1 instanceof Vector3) {
        return v1.distance(v2 as Vector3)
    } else {
        return v1.subtract(v2 as Vector2).length()
    }
}


//source: https://github.com/DerYeger/vecti/blob/master/src/vecti.ts
export class Vector2 {
    /**
     * Create a vector with the given components.
     * @param x - The component of the x-axis.
     * @param y - The component of the y-axis.
     * @returns The vector.
     */
    public static of([x, y]: [number, number]): Vector2 {
        return new Vector2(x, y)
    }

    /**
     * Create a vector with the given components.
     * @param x - The component of the x-axis.
     * @param y - The component of the y-axis.
     * @returns The vector.
     */
    public constructor(public readonly x: number, public readonly y: number) {}

    /**
     * Add another vector to the vector.
     * @param val - The vector to be added.
     * @returns The resulting vector of the addition.
     */
    public add(val: Vector2): Vector2 {
        return new Vector2(this.x + val.x, this.y + val.y)
    }

    /**
     * Subtract another vector from the vector.
     * @param val - The vector to be added.
     * @returns The resulting vector of the subtraction.
     */
    public subtract(val: Vector2): Vector2 {
        return new Vector2(this.x - val.x, this.y - val.y)
    }

    /**
     * Multiply the vector by a scalar.
     * @param scalar - The scalar the vector will be multiplied by.
     * @returns The resulting vector of the multiplication.
     */
    public multiply(scalar: number): Vector2 {
        return new Vector2(this.x * scalar, this.y * scalar)
    }

    /**
     * Divide the vector by a scalar.
     * @param scalar - The scalar the vector will be divided by.
     * @returns The resulting vector of the division.
     */
    public divide(scalar: number): Vector2 {
        return new Vector2(this.x / scalar, this.y / scalar)
    }

    /**
     * Calculate the dot product of the vector and another vector.
     * @param other - The other vector used for calculating the dot product.
     * @returns The dot product.
     */
    public dot(other: Vector2): number {
        return this.x * other.x + this.y * other.y
    }

    /**
     * Calculate the cross product of the vector and another vector. The cross product of two vectors `a` and `b` is defined as `a.x * b.y - a.y * b.x`.
     * @param other - The other vector used for calculating the cross product.
     * @returns The cross product.
     */
    public cross(other: Vector2): number {
        return this.x * other.y - other.x * this.y
    }

    /**
     * Calculate the Hadamard product of the vector and another vector.
     * @param other - The other vector used for calculating the Hadamard product.
     * @returns The Hadamard product.
     */
    public hadamard(other: Vector2): Vector2 {
        return new Vector2(this.x * other.x, this.y * other.y)
    }

    /**
     * Calculate the length of the vector using the L2 norm.
     * @returns The length.
     */
    public length(): number {
        return Math.sqrt(Math.pow(this.x, 2) + Math.pow(this.y, 2))
    }

    /**
     * Normalize the vector using the L2 norm.
     * @returns The normalized vector.
     */
    public normalize(): Vector2 {
        const length = this.length()
        return new Vector2(this.x / length, this.y / length)
    }

    /**
     * Rotate the vector by the given radians counterclockwise.
     * @param radians - The radians the vector will be rotated by.
     * @returns The rotated vector.
     */
    public rotateByRadians(radians: number): Vector2 {
        const cos = Math.cos(radians)
        const sin = Math.sin(radians)
        return new Vector2(this.x * cos - this.y * sin, this.x * sin + this.y * cos)
    }

    /**
     * Rotate the vector by the given degrees counterclockwise.
     * @param degrees - The degrees the vector will be rotated by.
     * @returns The rotated vector.
     */
    public rotateByDegrees(degrees: number): Vector2 {
        return this.rotateByRadians((degrees * Math.PI) / 180)
    }
}